// 文件或者目录实体
use serde::{Serialize};

use super::file_entry::FileEntry;

#[derive(Debug, Serialize)]
pub struct Source{

    entrys: Vec<FileEntry>,
    dir: String

}
impl Source {
    pub fn new(entrys: Vec<FileEntry>, dir: String) -> Self{
        Source { entrys, dir }
    }
}