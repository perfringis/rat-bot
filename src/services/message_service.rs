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

use crate::entities::message_entity::MessageEntity;
use crate::utils::Memory::Memory;
use crate::utils::Process::Process;

pub struct MessageService;

impl MessageService {
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

    pub fn read_sender_nickname(handle: HANDLE, address: usize) -> String {
        let address = MessageService::read_mem(handle, address + 0x39F1E50 as usize).unwrap();
        let address = MessageService::read_mem(handle, address + 0x70 as usize).unwrap();
        let address = MessageService::read_mem(handle, address + 0x20 as usize).unwrap();
        let address = MessageService::read_mem(handle, address + 0x18 as usize).unwrap();
        let address = MessageService::read_mem(handle, address + 0x28 as usize).unwrap();
        let address = MessageService::read_mem(handle, address + 0x28 as usize).unwrap();
        let address = MessageService::read_mem(handle, address + 0x38 as usize).unwrap();
        let address = MessageService::read_mem(handle, address + 0xD8 as usize).unwrap();
        let address = MessageService::read_mem(handle, address + 0x50 as usize).unwrap();
        let address = MessageService::read_mem(handle, address + 0x138 as usize).unwrap();

        let address = MessageService::read_mem(handle, address).unwrap();

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

    pub fn get_latest() -> MessageEntity {
        let process = Process::get_by_name("bf1").unwrap();

        let process_id = process.process_id;

        let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id as DWORD) };

        let module_handle = unsafe {
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id)
        };

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

        let nickname = MessageService::read_sender_nickname(process_handle, base_address);

        let is_chat_open = Memory::is_chat_open(process_handle, base_address);
        println!("MK TEST {:#?}", is_chat_open);

        let message = MessageEntity {
            sender: Some(nickname.split(":").next().unwrap().to_string()),
            content: Some(String::from("value")),
        };

        message
    }

    pub fn get_latest_sender() -> MessageEntity {
        let message = MessageEntity {
            sender: Some(String::from("value")),
            content: None,
        };

        message
    }

    pub fn get_latest_content() -> MessageEntity {
        let message = MessageEntity {
            sender: None,
            content: Some(String::from("value")),
        };

        message
    }
}
