#[cfg(windows)]
extern crate winres;
// use winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    // 设置图标
    res.set_icon(r"E:\remote_control_client-master\src\icon.ico");
    // 要求管理员权限
    res.set_manifest(r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
            </requestedPrivileges>
        </security>
    </trustInfo>
    </assembly>
    "#);
    res.compile().unwrap();
    
}

#[cfg(unix)]
fn main() {
}