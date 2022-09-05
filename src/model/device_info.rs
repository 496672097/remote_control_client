// 文件或者目录实体
use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct DeviceInfo{

    os: String,
    cpu_core: u32,
    memory_total: u64,
    memory_free: u64,
    memory_avail: u64,
    disk_total: u64,
    disk_free: u64,
    hostname: String

}

impl DeviceInfo {
    pub fn new(
        os: String,
        cpu_core: u32,
        memory_total: u64,
        memory_free: u64,
        memory_avail: u64,
        disk_total: u64,
        disk_free: u64,
        hostname: String) -> Self{
        DeviceInfo{ os, cpu_core, memory_total, memory_free, memory_avail, disk_total, disk_free, hostname }
    }
}