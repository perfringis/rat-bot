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
        println!("BASE ADDRESS: {:#?}", module_entry.modBaseAddr);
    }

    unsafe { CloseHandle(module_handle) };

    let base_address = module_entry.modBaseAddr;
    println!("BASE ADDRESS: {:#?}", base_address);

    let base_address_size = module_entry.modBaseSize;
    println!("BASE ADDRESS SIZE: {:#?}", base_address_size);

    let offset = 0x39F1E50 as usize;
    println!("OFFSET 1: {:#?}", offset);

    let base_address_usize = base_address as usize;
    println!("BASE ADDRESS CONVERTED TO USIZE: {:#?}", base_address_usize);

    let base_address_plus_offset = base_address_usize + offset;
    println!("BASE ADDRESS + OFFSET 1: {:#?}", base_address_plus_offset);

    // let mut buffer: usize = 0;
    let mut buffer = vec![0u8; 8];
    println!("BUFFER BEFORE READ PROCESS MEMORY 1: {:#?}", buffer);

    unsafe {
        ReadProcessMemory(
            process_handle as HANDLE,
            base_address_plus_offset as LPCVOID,
            buffer.as_mut_ptr() as LPVOID,
            size_of::<usize>() as SIZE_T,
            null_mut(),
        )
    };

    println!("BUFFER AFTER READ PROCESS MEMORY 1: {:#?}", buffer);
    println!(
        "BUFFER AFTER READ PROCESS MEMORY CONVERTED 1: {:#?}",
        usize::from_ne_bytes(buffer[..8].try_into().unwrap())
    );

    let offset = 0x70 as usize;
    println!("OFFSET 2: {:#?}", offset);

    let base_address_usize = usize::from_ne_bytes(buffer[..8].try_into().unwrap()) as usize;
    println!(
        "BASE ADDRESS CONVERTED TO USIZE 2: {:#?}",
        base_address_usize
    );

    let base_address_plus_offset = base_address_usize + offset;
    println!("BASE ADDRESS + OFFSET 2: {:#?}", base_address_plus_offset);

    buffer = vec![0u8; 8];
    println!("BUFFER BEFORE READ PROCESS MEMORY 2: {:#?}", buffer);

    unsafe {
        ReadProcessMemory(
            process_handle as HANDLE,
            base_address_plus_offset as LPCVOID,
            buffer.as_mut_ptr() as LPVOID,
            size_of::<usize>() as SIZE_T,
            null_mut(),
        )
    };

    println!("BUFFER AFTER READ PROCESS MEMORY 2: {:#?}", buffer);
    println!(
        "BUFFER AFTER READ PROCESS MEMORY CONVERTED 2: {:#?}",
        usize::from_ne_bytes(buffer[..8].try_into().unwrap())
    );
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
