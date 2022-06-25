use std::io::stdout;
use crossterm::{
    queue,
    cursor::{RestorePosition, SavePosition, MoveRight},
    style::{Print, PrintStyledContent, Stylize},
};

pub struct FetchData {
    pub label: &'static str,
    text: String,
}

impl FetchData {
    pub fn queue_print(&self, data_pos: u16) {
        queue!(
            stdout(),
            SavePosition,
            PrintStyledContent(self.label.bold().cyan()),
            RestorePosition,
            MoveRight(data_pos),
            Print(&self.text),
            Print("\n"),
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
