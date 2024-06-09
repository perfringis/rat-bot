#[cfg(windows)]
extern crate winapi;

use std::ffi::{OsStr, OsString};
use std::io::Read;
use std::iter::once;
use std::mem::size_of;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr::{self, null_mut};
use std::{io, mem};
use sysinfo::System;
use winapi::ctypes::{c_int, c_void};
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, LPARAM, LPCVOID, LPVOID, TRUE};
use winapi::shared::windef::HWND;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32};
use winapi::um::tlhelp32::{Module32FirstW, MODULEENTRY32W};
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::winuser::{
    EnumWindows, FindWindowW, GetClassNameW, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
};

use winapi::shared::ntdef::{HANDLE, NULL};
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::memoryapi::VirtualAllocEx;
use winapi::um::winnt::MEM_COMMIT;
use winapi::um::winnt::PAGE_READWRITE;

fn read_address(process: HANDLE, address: *mut u8) -> Result<Vec<u8>, u8> {
    let mut buffer = vec![0u8; 8];

    let address = unsafe {
        ReadProcessMemory(
            process,
            address as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len() as SIZE_T,
            0 as *mut SIZE_T,
        )
    };

    if address == 1 {
        Ok(buffer)
    } else {
        Err(0)
    }
}

fn chat_list_pointer(process: HANDLE, base_address: *mut u8) {
    let offset_address = base_address.wrapping_add(0x39F1E50);
    // let address = read_address(process, offset_address).unwrap();


    // new approach
    // let str_len = address.iter().position(|x| *x == 0).unwrap_or_default();
    // println!("{:#?}", String::from_utf8_lossy(&address[0..str_len]));

    // new approach 2
    // let address = address.as_ptr() as *const u32;
    // println!("addresS: {:#?}", address);

    // new approach 3
    // println!("{:#?}",  String::from_utf8_lossy(&address));

    // let mut buffer = vec![0u8; 8];
    // let mut buffer = [0u8; 8];

    // let address = unsafe {
    //     ReadProcessMemory(
    //         process,
    //         base_address as *const c_void,
    //         buffer.as_mut_ptr() as *mut c_void,
    //         buffer.len() as SIZE_T,
    //         0 as *mut SIZE_T,
    //     )
    // };
    // let address_test = usize::from_ne_bytes(buffer) as *mut c_void;
    // println!("TEXT: {:?}", address_test);

    // let mut buffer = 0usize;

    // let address = unsafe {
    //     ReadProcessMemory(
    //         process,
    //         base_address as *const c_void,
    //         &mut buffer as *mut usize as _,
    //         std::mem::size_of::<usize>(),
    //         0 as *mut SIZE_T,
    //     )
    // };
    // println!("lol: {:?}", buffer);

    // let mut address_buffer = [0; 8 as usize];
    //     let address = unsafe {
    //     ReadProcessMemory(
    //         process,
    //         base_address as *const c_void,
    //         address_buffer.as_mut_ptr() as *mut c_void,
    //         std::mem::size_of::<usize>(),
    //         0 as *mut SIZE_T,
    //     )
    // };
    // let pe_base_address = unsafe { ptr::read(address_buffer.as_ptr() as *const usize) };
    // println!("lol 2: {:?}", pe_base_address);
}

fn main() {
    let process_data = match find_process("bf") {
        Ok(process) => process,
        Err(_) => (0, String::new()),
    };

    let process_id = process_data.0;
    let process_name = process_data.1;

    println!("process id: {:#?}", process_id);
    println!("process name: {:#?}", process_name);

    let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id as DWORD) };
    println!("process handle: {:#?}", process_handle);

    // in other code is 0x300 = 768 = 256 * 3
    // let allocate_memory_address =
    //     unsafe { VirtualAllocEx(process_handle, NULL, 256 * 3, MEM_COMMIT, PAGE_READWRITE) };

    // println!("allocate_memory_address: {:#?}", allocate_memory_address);

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

    let base_address = module_entry.modBaseAddr;
    chat_list_pointer(process_handle, base_address);
    // let pointer = base_address;

    // let pointer = read_address(process_handle, pointer.wrapping_add(0x39f1e50)).unwrap();
    // println!("test {:#?}", pointer);
    // let pointer = pointer.as_ptr() as *mut u8;
    // println!("test {:#?}", pointer);

    // 4814670592
    // 4814670592
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
