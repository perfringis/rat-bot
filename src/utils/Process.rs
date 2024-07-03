use sysinfo::System;

pub struct DiskUsageData {
    pub total_written_bytes: u64,
    pub written_bytes: u64,
    pub total_read_bytes: u64,
    pub read_bytes: u64,
}

pub struct ProcessData {
    pub process_id: u32,
    pub parent_process_id: u32,
    pub process_name: String,
    pub environment_variables: Vec<String>,
    pub command_line: Vec<String>,
    pub executable_path: String,
    pub current_working_directory: String,
    pub memory_usage: u64,
    pub virtual_memory_usage: u64,
    pub cpu_usage: f32,
    pub status: String,
    pub root: String,
    pub disk_usage: DiskUsageData,
    pub user_id: String,
    pub effective_user_id: String,
    pub effective_group_id: String,
    pub group_id: String,
    pub running_time: u64,
}

pub struct Process;

impl Process {
    pub fn get_by_name(process_name: &str) -> Option<ProcessData> {
        let mut system = System::new_all();

        system.refresh_all();

        for process in system.processes().values() {
            if process.name().contains(process_name) {
                let process_data = ProcessData {
                    process_id: process.pid().as_u32(),
                    parent_process_id: process.parent().unwrap_or(0.into()).as_u32(),
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
                        None => String::new(),
                    },
                    effective_group_id: match process.effective_group_id() {
                        Some(group_id) => group_id.to_string(),
                        None => String::new(),
                    },
                    group_id: match process.group_id() {
                        Some(group_id) => group_id.to_string(),
                        None => String::new(),
                    },
                    running_time: process.run_time(),
                };

                return Some(process_data);
            }
        }
        None
    }
}
