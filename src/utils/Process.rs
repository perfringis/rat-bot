use sysinfo::System;

pub struct Process;

#[derive(Debug)]
pub struct DiskUsageData {
    total_written_bytes: u64,
    written_bytes: u64,
    total_read_bytes: u64,
    read_bytes: u64,
}

#[derive(Debug)]
pub struct ProcessData {
    process_id: u32,
    parent_process_id: u32,
    process_name: String,
    environment_variables: Vec<String>,
    command_line: Vec<String>,
    executable_path: String,
    current_working_directory: String,
    memory_usage: u64,
    virtual_memory_usage: u64,
    cpu_usage: f32,
    status: String,
    root: String,
    disk_usage: DiskUsageData,
    user_id: String,
    effective_user_id: String,
    effective_group_id: String,
    group_id: String,
    running_time: u64,
}

impl Process {
    pub fn get_by_name(process_name: &str) -> Option<ProcessData> {
        let mut system = System::new_all();

        system.refresh_all();

        for process in system.processes().values() {
            if process.name().contains(process_name) {
                let process_data: ProcessData = ProcessData {
                    process_id: process.pid().as_u32(),
                    parent_process_id: process.parent().unwrap().as_u32(),
                    process_name: process.name().to_string(),
                    environment_variables: process.environ().into(),
                    command_line: process.cmd().into(),
                    executable_path: process.exe().unwrap().display().to_string(),
                    current_working_directory: process.cwd().unwrap().display().to_string(),
                    memory_usage: process.memory(),
                    virtual_memory_usage: process.virtual_memory(),
                    cpu_usage: process.cpu_usage(),
                    status: process.status().to_string(),
                    root: process.root().unwrap().display().to_string(),
                    disk_usage: DiskUsageData {
                        total_written_bytes: process.disk_usage().total_written_bytes,
                        written_bytes: process.disk_usage().written_bytes,
                        total_read_bytes: process.disk_usage().total_read_bytes,
                        read_bytes: process.disk_usage().read_bytes,
                    },
                    user_id: process.user_id().unwrap().to_string(),
                    effective_user_id: match process.effective_user_id() {
                        Some(user_id) => user_id.to_string(),
                        None => String::from("None"),
                    },
                    effective_group_id: match process.effective_group_id() {
                        Some(group_id) => group_id.to_string(),
                        None => String::from("None"),
                    },
                    group_id: match process.group_id() {
                        Some(group_id) => group_id.to_string(),
                        None => String::from("None"),
                    },
                    running_time: process.run_time(),
                };

                return Some(process_data);
            }
        }
        None
    }
}
