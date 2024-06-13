#[cfg(windows)]
extern crate winapi;

use std::ffi::{CStr, OsStr, OsString};
use std::io::Read;
use std::iter::once;
use std::mem::size_of;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr::{self, null_mut};
use std::str::FromStr;
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

fn read_mem(handle: HANDLE, address: usize) -> Option<usize> {
    let mut buffer = vec![0u8; 8];

    let read_result = unsafe {
        ReadProcessMemory(
            handle as HANDLE,
            address as LPCVOID,
            buffer.as_mut_ptr() as LPVOID,
            size_of::<usize>() as SIZE_T,
            null_mut(),
        )
    };

    if read_result == 0 {
        None
    } else {
        Some(usize::from_ne_bytes(buffer[..8].try_into().unwrap()))
    }
}

fn read_sender_nickname(handle: HANDLE, address: usize) -> String {
    let address = read_mem(handle, address + 0x39F1E50 as usize).unwrap();
    let address = read_mem(handle, address + 0x70 as usize).unwrap();
    let address = read_mem(handle, address + 0x20 as usize).unwrap();
    let address = read_mem(handle, address + 0x18 as usize).unwrap();
    let address = read_mem(handle, address + 0x28 as usize).unwrap();
    let address = read_mem(handle, address + 0x28 as usize).unwrap();
    let address = read_mem(handle, address + 0x38 as usize).unwrap();
    let address = read_mem(handle, address + 0xD8 as usize).unwrap();
    let address = read_mem(handle, address + 0x50 as usize).unwrap();
    let address = read_mem(handle, address + 0x138 as usize).unwrap();

    let address = read_mem(handle, address).unwrap();

    let mut buffer = vec![0u8; 32];

    unsafe {
        ReadProcessMemory(
            handle as HANDLE,
            address as LPCVOID,
            buffer.as_mut_ptr() as LPVOID,
            32 as SIZE_T,
            null_mut(),
        )
    };

    let nickname = String::from_utf8_lossy(&buffer).to_string();
    nickname
}

fn read_sender_message(handle: HANDLE, address: usize) -> String {
    let address = read_mem(handle, address + 0x39F1E50 as usize).unwrap();
    let address = read_mem(handle, address + 0x70 as usize).unwrap();
    let address = read_mem(handle, address + 0x20 as usize).unwrap();
    let address = read_mem(handle, address + 0x18 as usize).unwrap();
    let address = read_mem(handle, address + 0x28 as usize).unwrap();
    let address = read_mem(handle, address + 0x28 as usize).unwrap();
    let address = read_mem(handle, address + 0x38 as usize).unwrap();
    let address = read_mem(handle, address + 0xD8 as usize).unwrap();
    let address = read_mem(handle, address + 0x50 as usize).unwrap();
    let address = read_mem(handle, address + 0x140 as usize).unwrap();

    let address = read_mem(handle, address).unwrap();

    let mut buffer = vec![0u8; 256];

    unsafe {
        ReadProcessMemory(
            handle as HANDLE,
            address as LPCVOID,
            buffer.as_mut_ptr() as LPVOID,
            256 as SIZE_T,
            null_mut(),
        )
    };

    let readable_message_len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());

    let message = String::from_utf8_lossy(&buffer[..readable_message_len]).to_string();
    message
}

fn get_server_name(handle: HANDLE, address: usize) -> String {
    let address = read_mem(handle, address + 0x3A1F3F8 as usize).unwrap();
    let address = read_mem(handle, address + 0x30 as usize).unwrap();

    let mut buffer = vec![0u8; 64];

    unsafe {
        ReadProcessMemory(
            handle as HANDLE,
            address as LPCVOID,
            buffer.as_mut_ptr() as LPVOID,
            64 as SIZE_T,
            null_mut(),
        )
    };

    let readable_message_len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
    let server_name = String::from_utf8_lossy(&buffer[..readable_message_len]).to_string();

    server_name
}

fn get_server_id(handle: HANDLE, address: usize) -> usize {
    let address = read_mem(handle, address + 0x37FF1A0 as usize).unwrap();
    let address = read_mem(handle, address + 0x418 as usize).unwrap();

    address
}

fn get_map_name(handle: HANDLE) -> String {
    let pointer = read_mem(handle, 0x1437F7758 as usize).unwrap();
    let pointer = read_mem(handle, pointer + 0x30 as usize).unwrap();
    let pointer = read_mem(handle, pointer + 0x18 as usize).unwrap();
    let pointer = read_mem(handle, pointer + 0xB0 as usize).unwrap();

    let mut buffer = vec![0u8; 64];

    unsafe {
        ReadProcessMemory(
            handle as HANDLE,
            pointer as LPCVOID,
            buffer.as_mut_ptr() as LPVOID,
            64 as SIZE_T,
            null_mut(),
        )
    };

    let readable_message_len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
    let map_name = String::from_utf8_lossy(&buffer[..readable_message_len]).to_string();

    map_name
}

fn get_game_mode(handle: HANDLE) -> String {
    let pointer = read_mem(handle, 0x1437F7758 as usize).unwrap();
    let pointer = read_mem(handle, pointer + 0x30 as usize).unwrap();
    let pointer = read_mem(handle, pointer + 0x30 as usize).unwrap();
    let pointer = read_mem(handle, pointer + 0x08 as usize).unwrap();

    let mut buffer = vec![0u8; 64];

    unsafe {
        ReadProcessMemory(
            handle as HANDLE,
            pointer as LPCVOID,
            buffer.as_mut_ptr() as LPVOID,
            64 as SIZE_T,
            null_mut(),
        )
    };

    let readable_message_len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
    let map_name = String::from_utf8_lossy(&buffer[..readable_message_len]).to_string();

    map_name
}

fn get_server_time(handle: HANDLE, address: usize) -> usize {
    let address = read_mem(handle, address + 0x3A31138 as usize).unwrap();
    let address = read_mem(handle, address + 0x20 as usize).unwrap();
    let address = read_mem(handle, address + 0x38 as usize).unwrap();
    let address = read_mem(handle, address + 0x58 as usize).unwrap();
    let address = read_mem(handle, address + 0x78 as usize).unwrap();

    address
}

fn get_server_score_ptr(handle: HANDLE, address: usize) -> usize {
    // this is pointer to server score

    let address = read_mem(handle, address + 0x3A0FC40 as usize).unwrap();
    let address = read_mem(handle, address + 0x58 as usize).unwrap();
    let address = read_mem(handle, address + 0x18 as usize).unwrap();
    let address = read_mem(handle, address + 0x08 as usize).unwrap();

    address
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
        println!("BASE ADDRESS: {:#?}", module_entry.modBaseAddr as usize);
    }

    unsafe { CloseHandle(module_handle) };

    let base_address = module_entry.modBaseAddr as usize;

    // let nickname = read_sender_nickname(process_handle, base_address);
    // println!("SENDER NICKNAME: {:#?}", nickname.split(":").next().unwrap().to_string());

    // let message = read_sender_message(process_handle, base_address);
    // println!("SENDER MESSAGE: {:#?}", message.trim());

    // let server_name = get_server_name(process_handle, base_address);
    // println!("SERVER NAME: {:#?}", server_name);

    // let server_id = get_server_id(process_handle, base_address);
    // println!("SERVER ID: {:#?}", server_id);

    // let map_name = get_map_name(process_handle);
    // println!("MAP NAME: {:#?}", map_name);

    // let game_mode = get_game_mode(process_handle);
    // println!("GAME MODE: {:#?}", game_mode);

    // let server_time = get_server_time(process_handle, base_address);
    // println!("SERVER TIME: {:#?}", server_time);

    let server_score_ptr = get_server_score_ptr(process_handle, base_address);
    println!("SERVER SCORE PTR: {:#?}", server_score_ptr);
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
