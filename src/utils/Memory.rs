use std::mem::{size_of, zeroed};
use std::ptr::null_mut;
use std::{io, mem};
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{DWORD, FALSE, LPCVOID, LPVOID, TRUE};
use winapi::shared::ntdef::HANDLE;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Module32FirstW, MODULEENTRY32W, TH32CS_SNAPMODULE,
    TH32CS_SNAPMODULE32,
};
use winapi::um::winnt::PROCESS_ALL_ACCESS;

// use super::Process::Process;
use crate::utils::Process::Process;

pub struct Memory {
    process_handle: HANDLE,
    snapshot_handle: HANDLE,
    base_address: usize,
}

impl Memory {
    // rewrite code remove parameter
    // write setters and replace the code
    pub fn init(&mut self) -> bool {
        let process = Process::get_by_name("bf1").unwrap();

        let process_id = process.process_id;

        unsafe {
            let process_handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id as DWORD);
            if process_handle.is_null() {
                return false;
            } else {
                self.process_handle = process_handle;
            }

            let snapshot_handle =
                CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id);

            if snapshot_handle.is_null() || snapshot_handle == INVALID_HANDLE_VALUE {
                CloseHandle(process_handle);

                return false;
            } else {
                self.snapshot_handle = snapshot_handle;
            }

            let mut module_entry: MODULEENTRY32W = mem::zeroed();
            module_entry.dwSize = mem::size_of::<MODULEENTRY32W>() as u32;

            let success = Module32FirstW(snapshot_handle, &mut module_entry);
            if success == FALSE {
                CloseHandle(process_handle);
                CloseHandle(snapshot_handle);

                return false;
            } else {
                self.base_address = module_entry.modBaseAddr as usize;
            }

            CloseHandle(process_handle);
            CloseHandle(snapshot_handle);
        }

        return true;
    }

    pub fn read(handle: HANDLE, address: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; 8];

        unsafe {
            ReadProcessMemory(
                handle as HANDLE,
                address as LPCVOID,
                buffer.as_mut_ptr() as LPVOID,
                size_of::<usize>() as SIZE_T,
                null_mut(),
            )
        };

        buffer
    }

    pub fn read_test<T>(handle: HANDLE, address: usize) -> T {
        unsafe {
            let mut buffer = zeroed();

            ReadProcessMemory(
                handle as HANDLE,
                address as LPCVOID,
                &mut buffer as *mut T as LPVOID,
                size_of::<T>(),
                null_mut(),
            );

            buffer
        }
    }

    // TODO check implementation with C# project
    pub fn convert_buffer_to_usize(buffer: Vec<u8>) -> usize {
        // let buf = buffer.try_into().unwrap_or([0,0,0,0,0,0,0,0]);
        let buf = buffer.try_into().unwrap();

        usize::from_ne_bytes(buf)
    }

    // TODO check implementation with C# project
    pub fn is_chat_open(handle: HANDLE, base_address: usize) -> bool {
        let address = Memory::read(handle, base_address + 0x39F1E50);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return false;
        }

        let address = Memory::read(handle, address + 0x08);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return false;
        }

        let address = Memory::read(handle, address + 0x28);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return false;
        }

        let address = Memory::read(handle, address + 0x00);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return false;
        }

        let address = Memory::read(handle, address + 0x20);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return false;
        }

        let address = Memory::read(handle, address + 0x18);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return false;
        }

        let address = Memory::read(handle, address + 0x28);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return false;
        }

        let address = Memory::read(handle, address + 0x38);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return false;
        }

        let address = Memory::read(handle, address + 0x40);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return false;
        }

        let address = Memory::read(handle, address + 0x30);
        let address = Memory::convert_buffer_to_usize(address);

        return address == 0x01;
    }

    pub fn get_chat_pointer(handle: HANDLE, base_address: usize) -> usize {
        let address = Memory::read(handle, base_address + 0x39F1E50);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return 0x0;
        }

        let address = Memory::read(handle, address + 0x70);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return 0x0;
        }

        let address = Memory::read(handle, address + 0x20);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return 0x0;
        }

        let address = Memory::read(handle, address + 0x18);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return 0x0;
        }

        let address = Memory::read(handle, address + 0x28);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return 0x0;
        }

        let address = Memory::read(handle, address + 0x28);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return 0x0;
        }

        let address = Memory::read(handle, address + 0x38);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return 0x0;
        }

        let address = Memory::read(handle, address + 0xD8);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return 0x0;
        }

        let address = Memory::read(handle, address + 0x50);
        let address = Memory::convert_buffer_to_usize(address);
        if !Memory::is_valid(address) {
            return 0x0;
        }

        address
    }

    pub fn is_valid(address: usize) -> bool {
        if address >= 0x10000 && address <= 0x000F000000000000 {
            true
        } else {
            false
        }
    }
}
