// 文件或者目录实体
use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct FileEntry {
    name: String,
    is_dir: bool,
}

impl FileEntry {
    pub fn new(name: String, is_dir: bool) -> Self{
        FileEntry{
            name, is_dir
        }
    }
}