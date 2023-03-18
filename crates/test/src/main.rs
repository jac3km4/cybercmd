use std::thread::sleep;
use widestring::U16CString;
use winapi::um::winver::GetFileVersionInfoW;
use winapi::um::processenv::GetCommandLineW;

pub fn main() {
    // We use get_file_version_info to create a dependency on version.dll so that the asi loader gets loaded.
    get_file_version_info();

    println!("Cybercmd test app.");
    println!();
    println!("By itself, this app does nothing but print this text.");
    println!("It is an empty placeholder for the asi loader and cybercmd for testing.");

    println!("Call GetCommandLineW, got: {}", get_command_line());

    sleep(std::time::Duration::from_millis(200));
}


fn get_file_version_info() {
    // This does nothing.
    let filename = U16CString::from_str_truncate(std::env::current_exe().unwrap().to_string_lossy());
    let mut buffer: [u8; 512] = [0; 512];
    unsafe { GetFileVersionInfoW(filename.as_ptr(), 0,512,buffer.as_mut_ptr().cast() ); }
}

fn get_command_line() -> String {
    let ret_val = unsafe {GetCommandLineW()};
    let win_string = unsafe {U16CString::from_ptr_str(ret_val)};
    win_string.to_string_lossy()
}
