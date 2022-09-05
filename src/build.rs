#[cfg(windows)]
extern crate winres;
// use winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon(r"E:\remote_control_client-master\src\icon.ico");
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {
}