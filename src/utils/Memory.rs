use winapi::{
    shared::{
        basetsd::SIZE_T,
        minwindef::{LPCVOID, LPVOID},
        ntdef::HANDLE,
    },
    um::memoryapi::ReadProcessMemory,
};

use std::{
    mem::{size_of, zeroed},
    ptr::null_mut,
};

pub struct Memory;

impl Memory {
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

        println!("MK TEST BUF {:#?}", buf);

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

    pub fn get_chat_pointer() {
        
    }

    pub fn is_valid(address: usize) -> bool {
        if address >= 0x10000 && address <= 0x000F000000000000 {
            true
        } else {
            false
        }
    }
}
