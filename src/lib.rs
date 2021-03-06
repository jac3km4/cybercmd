use anyhow::Result;
use detour::static_detour;
use microtemplate::{render, Substitutions};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::Write;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{ffi::CString, mem};
use widestring::{U16CStr, U16CString};
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, LPCWSTR};

#[derive(Substitutions, Clone)]
struct ConfigContext<'a> {
    game_dir: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct ModConfig {
    args: HashMap<String, String>,
    tasks: Vec<Task>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "command")]
pub enum Task {
    InvokeScc { path: String, custom_bundle: String },
}

static CMD_STR: Lazy<U16CString> = Lazy::new(try_get_final_cmd);

static_detour! {
  static GetCommandLineW: unsafe extern "system" fn() -> LPCWSTR;
}

type FnGetCommandLineW = unsafe extern "system" fn() -> LPCWSTR;

unsafe fn main() -> Result<(), Box<dyn Error>> {
    let address = get_module_symbol_address("kernel32.dll", "GetCommandLineW")
        .expect("could not find 'GetCommandLineW' address");
    let target: FnGetCommandLineW = mem::transmute(address);

    GetCommandLineW
        .initialize(target, || {
            if std::env::current_exe()
                .as_deref()
                .ok()
                .and_then(|path| path.file_stem())
                == Some(OsStr::new("Cyberpunk2077"))
            {
                CMD_STR.as_ptr()
            } else {
                GetCommandLineW.call()
            }
        })?
        .enable()?;
    Ok(())
}

fn try_get_final_cmd() -> U16CString {
    let initial_cmd_ustr = unsafe { U16CStr::from_ptr_str(GetCommandLineW.call()) };
    match get_final_cmd(initial_cmd_ustr) {
        Ok(res) => res,
        Err(_) => initial_cmd_ustr.to_owned(),
    }
}

fn get_final_cmd(initial_cmd_ustr: &U16CStr) -> Result<U16CString> {
    let mut cmd = initial_cmd_ustr.to_string()?;
    let path = get_game_path()?;
    let path = path.to_string_lossy();
    let ctx = ConfigContext {
        game_dir: path.as_ref(),
    };
    for config in get_configs()? {
        write_mod_cmd(&config, &ctx, &mut cmd)?;
        run_mod_tasks(&config, &ctx)?;
    }
    Ok(U16CString::from_str(cmd)?)
}

fn write_mod_cmd<W: Write>(config: &ModConfig, ctx: &ConfigContext, mut writer: W) -> Result<()> {
    for (key, val) in &config.args {
        let rendered = render(val, ctx.clone());
        write!(writer, " -{key} {rendered:?}")?;
    }
    Ok(())
}

fn run_mod_tasks(config: &ModConfig, ctx: &ConfigContext) -> Result<()> {
    for task in &config.tasks {
        match task {
            Task::InvokeScc {
                path,
                custom_bundle,
            } => {
                let cmd = Path::new(ctx.game_dir)
                    .join("engine")
                    .join("tools")
                    .join("scc.exe");
                let path = render(path, ctx.clone());
                let custom_bundle = render(custom_bundle, ctx.clone());
                const NO_WINDOW_FLAGS: u32 = 0x08000000;

                Command::new(&cmd)
                    .arg("-compile")
                    .arg(path)
                    .arg("-customPath")
                    .arg(custom_bundle)
                    .creation_flags(NO_WINDOW_FLAGS)
                    .status()
                    .ok();
            }
        }
    }
    Ok(())
}

fn get_game_path() -> Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let path = exe.parent().unwrap().parent().unwrap().parent().unwrap();
    Ok(path.to_path_buf())
}

fn get_configs() -> Result<Vec<ModConfig>> {
    let path = get_game_path()?
        .join("engine")
        .join("config")
        .join("cybercmd");
    let mut configs = vec![];

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        if entry.path().extension() == Some(OsStr::new("toml")) {
            let contents = std::fs::read(entry.path())?;
            configs.push(toml::from_slice(&contents)?);
        }
    }
    Ok(configs)
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
