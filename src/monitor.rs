use std::sync::{Arc, Mutex};
use std::time::Instant;

use serde::Serialize;
use sysinfo::{Pid, System};

#[derive(Debug, Clone, Serialize)]
pub struct SystemInfo {
    pub memory_mb: f64,
    pub memory_percent: f32,
    pub cpu_percent: f32,
    pub status: String,
    pub uptime_seconds: u64,
    pub pid: u32,
    pub total_memory_mb: f64,
    pub used_memory_mb: f64,
    pub total_tasks: usize,
    pub total_todo: usize,
    pub total_done: usize,
}

pub struct SystemMonitor {
    system: Arc<Mutex<System>>,
    pid: Pid,
    start_time: Instant,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        let pid = Pid::from(std::process::id() as usize);
        
        Self {
            system: Arc::new(Mutex::new(system)),
            pid,
            start_time: Instant::now(),
        }
    }

    pub fn get_info(
        &self,
        total_tasks: usize,
        total_todo: usize,
        total_done: usize,
    ) -> Option<SystemInfo> {
        let mut system = self.system.lock().unwrap();
        
        // Refresh only what we need
        system.refresh_memory();
        system.refresh_processes(
            sysinfo::ProcessesToUpdate::Some(&[sysinfo::Pid::from(std::process::id() as usize)]),
            true,
        );
        
        let process = system.process(self.pid)?;
        let total_memory = system.total_memory() as f64 / 1024.0 / 1024.0;
        let used_memory = system.used_memory() as f64 / 1024.0 / 1024.0;
        
        Some(SystemInfo {
            memory_mb: process.memory() as f64 / 1024.0 / 1024.0,
            memory_percent: process.memory() as f32 / system.total_memory() as f32 * 100.0,
            cpu_percent: process.cpu_usage(),
            status: format!("{:?}", process.status()),
            uptime_seconds: self.start_time.elapsed().as_secs(),
            pid: std::process::id(),
            total_memory_mb: total_memory,
            used_memory_mb: used_memory,
            total_tasks,
            total_todo,
            total_done,
        })
    }
}

impl Clone for SystemMonitor {
    fn clone(&self) -> Self {
        Self {
            system: Arc::clone(&self.system),
            pid: self.pid,
            start_time: self.start_time,
        }
    }
}
