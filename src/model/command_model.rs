// 传输指令的模型
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandModel{

    pub target: String,
    pub receiver: String,
    pub c_type: usize,
    pub action: usize,
    pub data: String,
    // pub data: T

}

impl CommandModel {
    
    pub fn new(target: String, receiver: String, c_type: usize, action: usize, data: String) -> CommandModel{
        CommandModel{
            target, receiver, c_type, action, data
        }
    }

}