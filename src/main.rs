#[cfg(windows)]
extern crate winapi;

use std::ffi::{OsStr, OsString};
use std::iter::once;
use std::mem;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr::null_mut;
use sysinfo::System;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, LPARAM, TRUE};
use winapi::shared::windef::HWND;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32};
use winapi::um::tlhelp32::{Module32FirstW, MODULEENTRY32W};
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::winuser::{
    EnumWindows, FindWindowW, GetClassNameW, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
};

use winapi::shared::ntdef::NULL;
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::memoryapi::VirtualAllocEx;
use winapi::um::winnt::MEM_COMMIT;
use winapi::um::winnt::PAGE_READWRITE;

fn chat_list_pointer() {}

fn main() {
    let process_data = match find_process("bf") {
        Ok(process) => process,
        Err(_) => (0, String::new()),
    };

    let process_id = process_data.0;
    let process_name = process_data.1;

    println!("process id: {:#?}", process_id);
    println!("process name: {:#?}", process_name);

    // 0x140000000 = 5368709120
    let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id as DWORD) };
    println!("process handle: {:#?}", process_handle);

    // in other code is 0x300 = 768 = 256 * 3
    let allocate_memory_address =
        unsafe { VirtualAllocEx(process_handle, NULL, 256 * 3, MEM_COMMIT, PAGE_READWRITE) };

    println!("allocate_memory_address: {:#?}", allocate_memory_address);

    // BASE ADDRESS START
    let module_handle =
        unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id) };

    if module_handle.is_null() || module_handle == INVALID_HANDLE_VALUE {
        println!("HANDLE IS NULL OR INVALID");
    }

    let mut module_entry: MODULEENTRY32W = unsafe { mem::zeroed() };
    module_entry.dwSize = mem::size_of::<MODULEENTRY32W>() as u32;

    let success = unsafe { Module32FirstW(module_handle, &mut module_entry) };
    if success == TRUE {
        println!("base address: {:#?}", module_entry.modBaseAddr);
    }

    unsafe { CloseHandle(module_handle) };
    // BASE ADDRESS END

    // add implementation of ReadProcessMemory
    // Win32.ReadProcessMemory(Bf1ProHandle, address, buffer, buffer.Length, out _);
    // Win32.ReadProcessMemory(Bf1ProHandle, address, buffer, size, out _);
    // ReadProcessMemory(hProcess, (LPCVOID)pointer, &value, sizeof(value), 0)
    // ReadProcessMemory(hProcess, (LPCVOID)(pointer + offset), &value, sizeof(value), 0)

    // let OFFSET_CHAT_MESSAGE_ADDRESS_START = 0x180;

    // different approach
    // let value = unsafe { module_entry.modBaseAddr.offset(0) as usize + 0x39f1e50 } as *mut u8;
    let offset = module_entry.modBaseAddr.wrapping_offset(0x39f1e50);
    println!("offset 1: {:#?}", offset);
    let offset = offset.wrapping_offset(0x8);
    println!("offset 2: {:#?}", offset);
    let offset = offset.wrapping_offset(0x28);
    println!("offset 3: {:#?}", offset);
    let offset = offset.wrapping_offset(0x0);
    println!("offset 4: {:#?}", offset);
    let offset = offset.wrapping_offset(0x20);
    println!("offset 5: {:#?}", offset);
    let offset = offset.wrapping_offset(0x18);
    println!("offset 6: {:#?}", offset);
    let offset = offset.wrapping_offset(0x28);
    println!("offset 7: {:#?}", offset);
    let offset = offset.wrapping_offset(0x38);
    println!("offset 8: {:#?}", offset);
    let offset = offset.wrapping_offset(0x40);
    println!("offset 9: {:#?}", offset);

    // add code here
    // let read_memory = unsafe { ReadProcessMemory(process_handle) };
}

fn get_base_address() {
    let process_data = match find_process("bf") {
        Ok(process) => process,
        Err(error) => (0, String::new()),
    };

    let process_id = process_data.0;

    let module_handle =
        unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id) };

    if module_handle.is_null() || module_handle == INVALID_HANDLE_VALUE {
        println!("HANDLE IS NULL OR INVALID");
    }

    let mut module_entry: MODULEENTRY32W = unsafe { mem::zeroed() };
    module_entry.dwSize = mem::size_of::<MODULEENTRY32W>() as u32;

    let success = unsafe { Module32FirstW(module_handle, &mut module_entry) };
    if success == TRUE {
        println!("{:#?}", module_entry.modBaseAddr);
    }

    unsafe { CloseHandle(module_handle) };
}

fn find_process(process_name: &str) -> Result<(u32, String), ()> {
    let mut system = System::new_all();

    system.refresh_all();

    for (pid, process) in system.processes() {
        if process.name().contains(process_name) {
            return Ok((pid.as_u32(), String::from(process.name())));
        }
    }

    Err(())
}

// fn find_window(window_name: &str) -> HWND {
//     let window_name: Vec<u16> = OsStr::new(window_name)
//         .encode_wide()
//         .chain(once(0))
//         .collect();
//     let hwnd: HWND = unsafe { FindWindowW(null_mut(), window_name.as_ptr()) };

//     hwnd
// }

// fn get_active_windows() -> Vec<(HWND, String, String)> {
//     unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
//         if IsWindowVisible(hwnd) != 0 {
//             let title_len = GetWindowTextLengthW(hwnd);

//             let mut title_buffer = Vec::with_capacity((title_len + 1) as usize);
//             title_buffer.set_len((title_len + 1) as usize);

//             GetWindowTextW(hwnd, title_buffer.as_mut_ptr(), title_len + 1);

//             let title = OsString::from_wide(&title_buffer);
//             let title = title.to_str().unwrap().to_string();

//             let mut class_buffer = Vec::with_capacity((32) as usize);
//             class_buffer.set_len((32) as usize);

//             GetClassNameW(hwnd, class_buffer.as_mut_ptr(), class_buffer.len() as c_int);

//             let class_name = OsString::from_wide(&class_buffer);
//             let class_name = class_name.to_str().unwrap().to_string();

//             let visible_windows: &mut Vec<(HWND, String, String)> =
//                 &mut *(lparam as *mut Vec<(HWND, String, String)>);
//             visible_windows.push((hwnd, title, class_name));
//         }

//         TRUE
//     }

//     let mut visible_windows: Vec<(HWND, String, String)> = Vec::new();
//     unsafe {
//         EnumWindows(
//             Some(enum_windows_callback),
//             &mut visible_windows as *mut _ as LPARAM,
//         );
//     }

//     visible_windows
// }
