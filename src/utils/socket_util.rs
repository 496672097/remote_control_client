use std::{net::TcpStream, io::Write};

use crate::model::command_model::CommandModel;

pub struct SocketUtil;

impl SocketUtil {
    pub fn send_to_server(json: String, stream: &mut TcpStream){
        match stream.write(json.as_bytes()) {
            Ok(_size) => (),
            Err(_err) => ()
            // {
            //     // SocketUtil::feed_to_server(String::from("转换返回数据json格式失败"), stream);
            //     print!("error ==> {}", err)
            // },
        }
    }

    pub fn send_error_server(msg: String, stream: &mut TcpStream){
        let model = CommandModel::new(
            stream.local_addr().unwrap().to_string(),
            String::from("*"), 
            usize::MIN,
            2,
            msg);
            // println!("error ==> {}", serde_json::to_string(&model).unwrap());
        // 封装一下错误信息
        SocketUtil::send_to_server(serde_json::to_string(&model).unwrap(), stream)
    }
}