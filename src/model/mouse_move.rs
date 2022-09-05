use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MouseMove{
    pub x: i32,
    pub y: i32
}