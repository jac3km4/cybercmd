use std::{
    ffi::CString, fmt::Write, mem, os::windows::process::CommandExt, path::Path, process::Command,
};

use anyhow::{bail, Result};
use common::path::PathBuf;
pub use config::AppContext;
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

use crate::{
    config::{ArgumentContext, GameConfig, Task},
    util::is_valid_exe,
};

pub mod config;
mod util;

static_detour! {
  static GetCommandLineW: unsafe extern "system" fn() -> LPCWSTR;
}

type FnGetCommandLineW = unsafe extern "system" fn() -> LPCWSTR;

static CMD_STR: Lazy<U16CString> = Lazy::new(try_get_final_cmd);

unsafe fn main() -> Result<()> {
    let address = get_module_symbol_address("kernel32.dll", "GetCommandLineW")
        .expect("could not find 'GetCommandLineW' address");
    let target: FnGetCommandLineW = mem::transmute(address);

    GetCommandLineW.initialize(target, || {
        if is_valid_exe() {
            CMD_STR.as_ptr()
        } else {
            GetCommandLineW.call()
        }
    })?;

    Lazy::force(&CMD_STR); // Run it before our hook, just in case.

    GetCommandLineW.enable()?;
    Ok(())
}

fn try_get_final_cmd() -> U16CString {
    let context = AppContext::new().expect("Could not load cybercmd");

    log::debug!("try_get_final_cmd");
    let initial_cmd_ustr = unsafe { U16CStr::from_ptr_str(GetCommandLineW.call()) };
    match get_final_cmd(&context, initial_cmd_ustr) {
        Ok(res) => res,
        Err(_) => initial_cmd_ustr.to_owned(),
    }
}

fn get_final_cmd(context: &AppContext, initial_cmd_ustr: &U16CStr) -> Result<U16CString> {
    log::debug!("get_final_cmd");
    let mut cmd = initial_cmd_ustr.to_string()?;

    for mod_config in &context.game_configs {
        write_mod_cmd(context, mod_config, &mut cmd)?;
        run_mod_tasks(context, mod_config);
    }
    Ok(U16CString::from_str(cmd)?)
}

fn write_mod_cmd<W: Write>(context: &AppContext, config: &GameConfig, mut writer: W) -> Result<()> {
    for (key, val) in &config.args {
        let rendered = render(val, context.argument_context.clone());
        write!(writer, " -{key} {rendered:?}")?;
    }
    Ok(())
}

fn run_mod_tasks(context: &AppContext, config: &GameConfig) {
    const NO_WINDOW_FLAGS: u32 = 0x0800_0000;

    for task in &config.tasks {
        log::debug!("Running task: \"{}\"", task.command);

        let cmd_path = get_command_path(context, task);
        let arg_context = ArgumentContext::from(context, &task.substitutions);

        let args = task
            .template_args
            .iter()
            .map(|arg| render(arg.as_str(), arg_context.clone()));

        if let Ok(cmd) = cmd_path {
            if cmd.starts_with(&context.paths.tools) || task.command == "InvokeScc" {
                let res = {
                    let mut command: Command = Command::new(&cmd);
                    command.args(args).current_dir(&context.paths.game);
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
                "Task \"{}\" from {} not found. ({:?})",
                task.command,
                config.file_name,
                if let Ok(path) = cmd_path {
                    path.as_os_str().to_string_lossy().to_string()
                } else {
                    String::new()
                }
            );
            if task.terminate_on_errors {
                std::process::exit(0);
            }
        }
    }
}

fn get_command_path(context: &AppContext, task: &Task) -> Result<PathBuf> {
    if task.command == "InvokeScc" {
        return Ok(context.paths.scc.normalize()?);
    }
    let cmd_with_exe = format!("{}.exe", task.command);

    let result = context
        .paths
        .tools
        .join(
            if let Some(file_name) = Path::new(&cmd_with_exe).file_name() {
                file_name
            } else {
                bail!("Cannot parse command");
            },
        )
        .normalize()?;

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
    if call_reason == DLL_PROCESS_ATTACH && is_valid_exe() {
        return i32::from(main().is_ok());
    }
    TRUE
}
