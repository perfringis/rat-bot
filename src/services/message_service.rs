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
use crate::utils::Offsets::OFFSET_CHAT_LAST_SENDER;
use crate::utils::Process::Process;

pub struct MessageService;

impl MessageService {
    pub fn read_sender_nickname(handle: HANDLE, base_address: usize) -> String {
        let address = Memory::get_chat_pointer(handle, base_address);

        let address = Memory::read(handle, address + OFFSET_CHAT_LAST_SENDER);
        let address = Memory::convert_buffer_to_usize(address);

        let address = Memory::read(handle, address);
        let address = Memory::convert_buffer_to_usize(address);

        let address = Memory::read_test::<[u8; 32]>(handle, address);

        String::from_utf8_lossy(&address).to_string()
    }

    pub fn get_latest() -> MessageEntity {
        // fix it if you rewrite the code
        Memory::init();

        // let nickname = MessageService::read_sender_nickname(process_handle, base_address);

        let message = MessageEntity {
            sender: Some(String::from("value")),
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
