use std::{ffi::CString, mem};
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fmt::Write;
use std::fs;
use std::ops::DerefMut;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use anyhow::Result;
use detour::static_detour;
use microtemplate::{render, Substitutions};
use once_cell::sync::Lazy;
use serde::Deserialize;
use widestring::{U16CStr, U16CString};
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, LPCWSTR};

use crate::logger::Logger;

pub mod logger;
pub mod paths;

#[derive(Substitutions, Clone)]
struct ConfigContext<'a> {
    game_dir: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct ModConfig {
    #[serde(default)]
    args: HashMap<String, String>,
    #[serde(default)]
    tasks: Vec<Task>,
}

fn default_as_false() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct Task {
    command: String,
    #[serde(default)]
    path: String,
    #[serde(default)]
    custom_cache_dir: String,
    #[serde(default = "default_as_false")]
    terminate_on_errors: bool,
}

static_detour! {
  static GetCommandLineW: unsafe extern "system" fn() -> LPCWSTR;
}

type FnGetCommandLineW = unsafe extern "system" fn() -> LPCWSTR;

unsafe fn main() -> Result<()> {
    let address = get_module_symbol_address("kernel32.dll", "GetCommandLineW")
        .expect("could not find 'GetCommandLineW' address");
    let target: FnGetCommandLineW = mem::transmute(address);

    static LOG: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::new()));
    static CMD_STR: Lazy<U16CString> =
        Lazy::new(|| try_get_final_cmd(LOG.lock().unwrap().deref_mut()));

    GetCommandLineW
        .initialize(target, || {
            let exe = std::env::current_exe();
            let stem = exe.as_deref().ok().and_then(Path::file_stem);
            if matches!(stem, Some(exe) if exe.eq_ignore_ascii_case("Cyberpunk2077")) {
                CMD_STR.as_ptr()
            } else {
                GetCommandLineW.call()
            }
        })?
        .enable()?;
    Ok(())
}

fn try_get_final_cmd(log: &mut Logger) -> U16CString {
    log.debug("try_get_final_cmd");
    let initial_cmd_ustr = unsafe { U16CStr::from_ptr_str(GetCommandLineW.call()) };
    match get_final_cmd(initial_cmd_ustr, log) {
        Ok(res) => res,
        Err(_) => initial_cmd_ustr.to_owned(),
    }
}

fn get_final_cmd(initial_cmd_ustr: &U16CStr, log: &mut Logger) -> Result<U16CString> {
    log.debug("get_final_cmd");
    let mut cmd = initial_cmd_ustr.to_string()?;
    let path = paths::get_game_path()?;
    let path = path.to_string_lossy();
    let ctx = ConfigContext {
        game_dir: path.as_ref(),
    };
    for config in get_configs(log)? {
        write_mod_cmd(&config, &ctx, &mut cmd)?;
        run_mod_tasks(&config, &ctx, log)?;
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

fn run_mod_tasks(config: &ModConfig, ctx: &ConfigContext, log: &mut Logger) -> Result<()> {
    static TASKS_DONE: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
    let cybercmd_path = Path::new(ctx.game_dir).join("tools").join("cybercmd");
    fs::create_dir_all(&cybercmd_path).unwrap();
    let cybercmd_path = cybercmd_path.canonicalize().unwrap();
    let cybercmd_path = cybercmd_path.to_str().unwrap();
    const NO_WINDOW_FLAGS: u32 = 0x08000000;

    for task in &config.tasks {
        let mut tasks_done = TASKS_DONE.lock().unwrap();
        if tasks_done.insert(task.command.clone()) {
            log.debug(format!("Running task: \"{}\"", task.command));

            match task.command.as_str() {
                "InvokeScc" => {
                    let cmd = Path::new(ctx.game_dir)
                        .join("engine")
                        .join("tools")
                        .join("scc.exe");
                    let path = render(task.path.as_str(), ctx.clone());
                    let custom_bundle = render(task.custom_cache_dir.as_str(), ctx.clone());

                    let res = Command::new(&cmd)
                        .arg("-compile")
                        .arg(path)
                        .arg("-customCacheDir")
                        .arg(custom_bundle)
                        .creation_flags(NO_WINDOW_FLAGS)
                        .status()
                        .ok();
                    if task.terminate_on_errors && !matches!(res, Some(st) if st.success()) {
                        std::process::exit(0);
                    }
                }
                _ => {
                    let cmd_path = Path::new(cybercmd_path)
                        .join(format!("{}.exe", task.command));
                    let cmd = cmd_path.canonicalize();

                    if let Ok(cmd) = cmd {
                        if cmd.starts_with(cybercmd_path) {
                            let res = Command::new(&cmd)
                                .arg("-cybercmd")
                                .current_dir(ctx.game_dir)
                                .creation_flags(NO_WINDOW_FLAGS)
                                .status()
                                .ok();
                            if !matches!(res, Some(st) if st.success()) {
                                log.error(format!("Task \"{}\" failed when run.", task.command));
                                if task.terminate_on_errors {
                                    std::process::exit(0);
                                }
                            }
                        } else {
                            log.error(format!("Task \"{}\" in invalid location.", task.command));
                            if task.terminate_on_errors {
                                std::process::exit(0);
                            }
                        }
                    } else {
                        log.error(format!(
                            "Task \"{}\" not found. ({:?})",
                            task.command,
                            dunce::simplified(cmd_path.as_path())
                        ));
                        if task.terminate_on_errors {
                            std::process::exit(0);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_configs(log: &mut Logger) -> Result<Vec<ModConfig>> {
    let path = paths::get_game_path()?
        .join("r6")
        .join("config")
        .join("cybercmd");
    let mut configs = vec![];

    log.debug("Getting configs.");

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.path().extension() == Some(OsStr::new("toml")) {
            log.debug(format!(
                "Loading: {}",
                dunce::simplified(&entry.path()).display()
            ));
            let contents = fs::read_to_string(entry.path())?;
            match toml::from_str(&contents) {
                Ok(config) => configs.push(config),
                Err(error) => log.error(format!(
                    "In {} ({}): {}",
                    dunce::simplified(&entry.path()).display(),
                    match error.span() {
                        Some(val) => format!("{:?}", val),
                        None => "".to_string(),
                    },
                    error.message()
                )),
            };
        }
    }
    Ok(configs)
}

pub fn log_dir() -> PathBuf {
    let log_dir = (match paths::get_game_path() {
        Ok(game_path) => game_path,
        Err(e) => panic!("cybercmd cannot get game path: {}", e),
    })
        .join("r6")
        .join("logs");
    match fs::create_dir_all(log_dir.clone()) {
        Ok(_) => log_dir,
        Err(e) => panic!("cybercmd cannot get game path: {}", e),
    }
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
#[cfg_attr(feature = "cargo-clippy", allow(clippy::missing_safety_doc))]
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
