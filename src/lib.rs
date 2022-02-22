use configparser::ini::Ini;
use detour::static_detour;
use microtemplate::{render, Substitutions};
use std::borrow::Cow;
use std::error::Error;
use std::path::PathBuf;
use std::{ffi::CString, mem};
use widestring::{U16CStr, U16CString};
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, LPCWSTR};

#[derive(Substitutions, Clone)]
struct ConfigContext<'a> {
    game_dir: Cow<'a, str>,
}

lazy_static::lazy_static! {
  static ref CMD_STR: U16CString = get_cmd_str();
  static ref CONFIG: Ini = get_config();
}

static_detour! {
  static GetCommandLineW: unsafe extern "system" fn() -> LPCWSTR;
}

type FnGetCommandLineW = unsafe extern "system" fn() -> LPCWSTR;

unsafe fn main() -> Result<(), Box<dyn Error>> {
    let address = get_module_symbol_address("kernel32.dll", "GetCommandLineW")
        .expect("could not find 'GetCommandLineW' address");
    let target: FnGetCommandLineW = mem::transmute(address);

    GetCommandLineW
        .initialize(target, || CMD_STR.as_ptr())?
        .enable()?;
    Ok(())
}

fn get_cmd_str() -> U16CString {
    let initial_cmd = unsafe { GetCommandLineW.call() };
    let initial_cmd_ustr = unsafe { U16CStr::from_ptr_str(initial_cmd) };
    let mut cmd = initial_cmd_ustr.to_string_lossy();
    let ctx = get_context();

    let conf_map = CONFIG.get_map_ref();
    if let Some(args) = conf_map.get("args") {
        for (key, val) in args {
            match val {
                Some(val) => {
                    let rendered = render(val, ctx.clone());
                    cmd.push_str(&format!(" -{key} {rendered}"))
                }
                None => cmd.push_str(&format!(" -{key}")),
            }
        }
    }

    U16CString::from_str_truncate(cmd)
}

fn get_context<'a>() -> ConfigContext<'a> {
    let exe = std::env::current_exe().unwrap();
    let path = exe.parent().unwrap().parent().unwrap().parent().unwrap();
    ConfigContext {
        game_dir: Cow::Owned(path.to_string_lossy().into_owned()),
    }
}

fn get_config() -> Ini {
    let mut ini = Ini::new_cs();
    let path = PathBuf::from(get_context().game_dir.as_ref())
        .join("engine")
        .join("config")
        .join("cmd.ini");
    if let Err(_) = ini.load(path) {
        // TODO
    }
    ini
}

unsafe fn get_module_symbol_address(module: &str, symbol: &str) -> Option<usize> {
    let module = U16CString::from_str_truncate(module);
    let symbol = CString::new(symbol).ok()?;
    let handle = GetModuleHandleW(module.as_ptr());
    match GetProcAddress(handle, symbol.as_ptr()) as usize {
        0 => None,
        n => Some(n),
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    _module: HINSTANCE,
    call_reason: DWORD,
    _reserved: LPVOID,
) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        main().is_ok() as BOOL
    } else {
        TRUE
    }
}
