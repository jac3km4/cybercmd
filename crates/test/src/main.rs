use std::thread::sleep;

use widestring::U16CString;
use winapi::um::{processenv::GetCommandLineW, winver::GetFileVersionInfoW};

pub fn main() {
    // We use get_file_version_info to create a dependency on version.dll so that the asi loader gets loaded.
    get_file_version_info();

    println!();
    println!("Cybercmd test app.");
    println!("exe: {:?}", std::env::current_exe().unwrap_or_default());
    println!("cwd: {:?}", std::env::current_dir().unwrap_or_default());
    println!();
    println!("Call GetCommandLineW, got:");
    println!("$ {}", get_command_line());

    sleep(std::time::Duration::from_millis(200));
}

fn get_file_version_info() {
    // This does nothing.
    let filename =
        U16CString::from_str_truncate(std::env::current_exe().unwrap().to_string_lossy());
    let mut buffer: [u8; 512] = [0; 512];
    unsafe {
        GetFileVersionInfoW(filename.as_ptr(), 0, 512, buffer.as_mut_ptr().cast());
    }
}

fn get_command_line() -> String {
    let ret_val = unsafe { GetCommandLineW() };
    let win_string = unsafe { U16CString::from_ptr_str(ret_val) };
    win_string.to_string_lossy()
}
