use std::net::TcpStream;
use sys_info::{mem_info, disk_info, cpu_num, hostname};
use crate::{model::{device_info::DeviceInfo, command_model::CommandModel}, utils::socket_util::SocketUtil};
pub struct QueryUtil;

impl QueryUtil {
    fn try_catch_err() -> Result<String, Box<dyn std::error::Error>>{
        let mut os = String::from("unknow");
        if cfg!(target_os = "windows") {
            os = String::from("windows");
        }else if cfg!(target_os = "linux"){
            os = String::from("linux");
        }else if cfg!(target_os = "macos"){
            os = String::from("macos");
        }
        let mem = mem_info()?;
        let disk = disk_info()?;
        let device_info = DeviceInfo::new(
            os,
            cpu_num()?,
            mem.total,
            mem.free,
            mem.avail,
            disk.total,
            disk.free,
            hostname()?
        );
        let data: String = serde_json::to_string(&device_info)?;
        Ok(data)
    }
    pub fn query_info(stream: &mut TcpStream){
        match QueryUtil::try_catch_err() {
            Ok(data) => {
                let info = CommandModel::new(
                    String::from("*"),
                    String::from("*"),
                    usize::MIN,
                    usize::MIN,
                    data
                );
                let data = serde_json::to_string(&info).unwrap();
                SocketUtil::send_to_server(data, stream);
            },
            Err(err) => SocketUtil::send_error_server(err.to_string(), stream)
        }
        
    }
}