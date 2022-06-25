pub fn get_os() -> String {
    let os = sys_info::os_type().unwrap_or_else(|_| "Unknown".to_string());
    if os.as_str() == "Linux" {
        match sys_info::linux_os_release() {
            Ok(info) => info.id.unwrap_or_else(|| "linux".to_string()),
            Err(_) => "linux".to_string(),
        }
    } else {
        os.to_lowercase()
    }
}
