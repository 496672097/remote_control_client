# remote_control_client

#### 介绍
采用rust语言编写的客户端程序

#### 使用说明
1.安装rust开发环境->https://www.rust-lang.org/zh-CN/learn/get-started
2.拉取项目通过cargo build --release编译项目获得可执行文件

#### 关于权限维持
默认采用写入用户登录即触发（需要管理员权限），可以修改为空闲时间触发（不需要管理员权限）需要修改build.rs文件和other.rs，通过注释的提示来进行修改

#### 其他
项目中摄像头拍照仅win可用，且和计划任务第三方库组合存在bug，暂时注释拍照相关代码。(可自行解开注释相关代码进行测试)

可执行文件图标替换icon.ico即可

其他问题可关注公众号“独语小栈”交流
