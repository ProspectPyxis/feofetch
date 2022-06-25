use crate::{
    config::{self, Config, FetchType},
    packages,
};
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
                text: config::get_os(),
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
            FetchType::Uptime => FetchData {
                label: "uptime",
                text: match uptime_lib::get() {
                    Ok(time) => {
                        let time = time.as_secs();
                        let mut timestr = String::new();
                        if time >= 3600 {
                            timestr.push_str(&format!("{}h ", time / 3600));
                        }
                        if time % 3600 > 0 {
                            timestr.push_str(&format!("{}m", (time % 3600) / 60));
                        }
                        if timestr.is_empty() {
                            timestr = "0m".to_string();
                        }
                        timestr
                    }
                    Err(_) => "unknown".to_string(),
                },
            },
            FetchType::Packages => FetchData {
                label: "packages",
                text: packages::get_packages(),
            },
            #[allow(unreachable_patterns)]
            _ => todo!(),
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
