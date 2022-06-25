use crate::config::{Config, FetchType};
use crossterm::{
    cursor::{MoveRight, RestorePosition, SavePosition},
    queue,
    style::{Print, PrintStyledContent, Stylize},
};
use std::io::stdout;

pub struct FetchData {
    pub label: &'static str,
    text: String,
}

impl FetchData {
    pub fn get(data: &FetchType) -> Self {
        match data {
            FetchType::Os => FetchData {
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
                },
            },
            FetchType::Version => FetchData {
                label: "version",
                text: {
                    let mut version =
                        sys_info::os_release().unwrap_or_else(|_| "unknown".to_string());
                    if let Some(index) = version.find('-') {
                        version.truncate(index);
                    }
                    version
                },
            },
        }
    }

    pub fn queue_print(&self, data_pos: u16) {
        queue!(
            stdout(),
            SavePosition,
            PrintStyledContent(self.label.bold().cyan()),
            RestorePosition,
            MoveRight(data_pos),
            Print(&self.text),
            Print("\n"),
        )
        .unwrap();
    }
}

pub fn fetch_all(conf: &Config) -> Vec<FetchData> {
    conf.data.iter().map(FetchData::get).collect()
}

pub fn print_all_fetches(data: &[FetchData], conf: &Config) {
    let max_label_len = data.iter().fold(0, |acc, x| {
        acc.max(
            x.label
                .chars()
                .count()
                .try_into()
                .unwrap_or(u16::MAX - conf.align_spaces)
                + conf.align_spaces,
        )
    });
    for d in data {
        d.queue_print(max_label_len);
    }
}
