#[cfg(windows)]
extern crate winapi;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{BOOL, LPARAM, TRUE};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{
    EnumWindows, GetClassNameW, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
};

fn main() {
    println!("{:#?}", get_active_windows());
}

fn get_active_windows() -> Vec<(HWND, String, String)> {
    unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        if IsWindowVisible(hwnd) != 0 {
            let title_len = GetWindowTextLengthW(hwnd);

            let mut title_buffer = Vec::with_capacity((title_len + 1) as usize);
            title_buffer.set_len((title_len + 1) as usize);

            GetWindowTextW(hwnd, title_buffer.as_mut_ptr(), title_len + 1);

            let title = OsString::from_wide(&title_buffer);
            let title = String::from(title.to_str().unwrap());
            // let title = String::from(title.to_string_lossy());

            let mut class_buffer = Vec::with_capacity((32) as usize);
            class_buffer.set_len((32) as usize);

            GetClassNameW(hwnd, class_buffer.as_mut_ptr(), class_buffer.len() as c_int);

            let class_name = OsString::from_wide(&class_buffer);
            let class_name = String::from(class_name.to_str().unwrap());
            // let class_name = String::from(class_name.to_string_lossy());

            let visible_windows: &mut Vec<(HWND, String, String)> =
                &mut *(lparam as *mut Vec<(HWND, String, String)>);
            visible_windows.push((hwnd, title, class_name));
        }

        TRUE
    }

    let mut visible_windows: Vec<(HWND, String, String)> = Vec::new();
    unsafe {
        EnumWindows(
            Some(enum_windows_callback),
            &mut visible_windows as *mut _ as LPARAM,
        );
    }

    visible_windows
}
