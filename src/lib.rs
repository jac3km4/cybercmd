use std::{
    ffi::CString,
    fmt::Write,
    mem,
    os::windows::process::CommandExt,
    path::{Path},
    process::Command,
};

use common::PathBuf;

use anyhow::{bail, Result};
use config::{ConfigContext, ModConfig};
use detour::static_detour;
use microtemplate::render;
use once_cell::sync::Lazy;
use widestring::{U16CStr, U16CString};
use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::{
        libloaderapi::{GetModuleHandleW, GetProcAddress},
        winnt::{DLL_PROCESS_ATTACH, LPCWSTR},
    },
};

use crate::{config::Task, paths::PATHS};

pub mod config;
pub mod paths;
pub mod util;

static_detour! {
  static GetCommandLineW: unsafe extern "system" fn() -> LPCWSTR;
}

type FnGetCommandLineW = unsafe extern "system" fn() -> LPCWSTR;

static CMD_STR: Lazy<U16CString> = Lazy::new(try_get_final_cmd);

unsafe fn main() -> Result<()> {
    let _ = util::setup_logging();
    let address = get_module_symbol_address("kernel32.dll", "GetCommandLineW")
        .expect("could not find 'GetCommandLineW' address");
    let target: FnGetCommandLineW = mem::transmute(address);

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

fn try_get_final_cmd() -> U16CString {
    log::debug!("try_get_final_cmd");
    let initial_cmd_ustr = unsafe { U16CStr::from_ptr_str(GetCommandLineW.call()) };
    match get_final_cmd(initial_cmd_ustr) {
        Ok(res) => res,
        Err(_) => initial_cmd_ustr.to_owned(),
    }
}

fn get_final_cmd(initial_cmd_ustr: &U16CStr) -> Result<U16CString> {
    log::debug!("get_final_cmd");
    let mut cmd = initial_cmd_ustr.to_string()?;

    for config in config::get_configs()? {
        write_mod_cmd(&config, &mut cmd)?;
        run_mod_tasks(&config)?;
    }
    Ok(U16CString::from_str(cmd)?)
}

fn write_mod_cmd<W: Write>(config: &ModConfig, mut writer: W) -> Result<()> {
    for (key, val) in &config.args {
        let rendered = render(val, ConfigContext{});
        write!(writer, " -{key} {rendered:?}")?;
    }
    Ok(())
}

fn run_mod_tasks(config: &ModConfig) -> Result<()> {
    const NO_WINDOW_FLAGS: u32 = 0x08000000;

    for task in &config.tasks {
        log::debug!("Running task: \"{}\"", task.command);

        let cmd_path = get_command_path(task);

        let args = task
            .args
            .iter()
            .map(|arg| render(arg.as_str(), task.clone()));

        if let Ok(cmd) = cmd_path {
            if cmd.starts_with(&PATHS.tools) || task.command == "InvokeScc" {
                let res = {
                    let mut command: Command = Command::new(&cmd);
                    command.args(args).current_dir(&PATHS.game);
                    if task.no_window {
                        command.creation_flags(NO_WINDOW_FLAGS);
                    }
                    log::info!("Run: {:?}", command);
                    command.status().ok()
                };
                if matches!(res, Some(st) if st.success()) {
                    log::info!("Task \"{}\" completed successfully!", task.command);
                } else {
                    log::error!("Task \"{}\" failed when run.", task.command);
                    if task.terminate_on_errors {
                        std::process::exit(0);
                    }
                }
            } else {
                log::error!("Task \"{}\" in invalid location.", task.command);
                if task.terminate_on_errors {
                    std::process::exit(0);
                }
            }
        } else {
            log::error!(
                "Task \"{}\" not found. ({:?})",
                task.command,
                if let Ok(path) = cmd_path {
                    path.as_os_str().to_string_lossy().to_string()
                } else {String::new()}
            );
            if task.terminate_on_errors {
                std::process::exit(0);
            }
        }
    }
    Ok(())
}

fn get_command_path(task: &Task) -> Result<PathBuf> {
    let result = if task.command == "InvokeScc" {
        PATHS.scc.to_owned()
    } else {
        let cmd_with_exe = format!("{}.exe", task.command);
        PATHS.tools.join(
            if let Some(file_name) = Path::new(&cmd_with_exe).file_name() {
                file_name
            } else {
                bail!("Cannot parse command");
            },
        )
    }
    .canonicalize()?;

    Ok(result)
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
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn DllMain(
    _module: HINSTANCE,
    call_reason: DWORD,
    _reserved: LPVOID,
) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        let exe = std::env::current_exe();
        let stem = exe.as_deref().ok().and_then(Path::file_stem);
        if matches!(stem, Some(exe) if exe.eq_ignore_ascii_case("Cyberpunk2077")) {
            return main().is_ok() as BOOL;
        }
    }
    TRUE
}
