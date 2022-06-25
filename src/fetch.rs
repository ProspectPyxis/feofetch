use std::io::stdout;
use crossterm::{
    queue,
    style::{Print, PrintStyledContent, Stylize},
};

pub struct FetchData {
    label: &'static str,
    text: String,
}

impl FetchData {
    pub fn queue_print(&self) {
        queue!(
            stdout(),
            PrintStyledContent(self.label.bold().cyan()),
            Print(format!(" {}", self.text)),
        ).unwrap();
    }
}

pub fn get_os() -> FetchData {
    FetchData {
        label: "os",
        text: {
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
    }
}
