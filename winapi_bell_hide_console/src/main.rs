#![windows_subsystem = "windows"]

use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

// extern crate kernel32;
extern crate libc;
// extern crate user32;
extern crate winapi;

// use user32::*;
// use winapi::winuser::*;
// use winapi::*;

// use winapi::minwindef::HINSTANCE;
// use winapi::windef::HBRUSH;
// use winapi::windef::HMENU;
// use winapi::windef::HWND;

// use winapi::minwindef::DWORD;
// use winapi::minwindef::LPARAM;
// use winapi::minwindef::LRESULT;
// use winapi::minwindef::UINT;
// use winapi::minwindef::WPARAM;
// use winapi::winnt::LPCWSTR;

// use winapi::winuser::WNDCLASSW;
// use winapi::winuser::WS_OVERLAPPEDWINDOW;
// use winapi::winuser::WS_VISIBLE;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

// fn hide_console_window() {
//     let window = unsafe { kernel32::GetConsoleWindow() };
//     if window != std::ptr::null_mut() {
//         unsafe { user32::ShowWindow(window, winapi::SW_HIDE) };
//     }
// }

fn main() {
    let mut cnt = 0u32;
    let stop_time = SystemTime::now() + Duration::new(10, 0);
    loop {
        let now = SystemTime::now();
        if now >= stop_time {
            break;
        }
        sleep(Duration::new(1, 0));
        print!(".");
        // print!("\x07");
        let _ = stdout().flush();

        cnt += 1;
        unsafe {
            let code: u32 = match cnt % 10 {
                1 => 0xFFFFFFFF,
                2 => winapi::MB_ICONSTOP,
                3 => winapi::MB_ICONERROR,
                4 => winapi::MB_ICONQUESTION,
                5 => winapi::MB_ICONINFORMATION,
                6 => winapi::MB_ICONWARNING,
                7 => winapi::MB_ICONHAND,
                8 => winapi::MB_OK,
                _ => 0,
            };
            let err = user32::MessageBeep(code);
            println!("{}: ret from MessageBeep({}) => {}", cnt, code, err);
        }
    }
}
