use crate::{
    config::{self, Config, FetchType},
    packages,
};
use crossterm::{
    cursor::{MoveRight, RestorePosition, SavePosition},
    queue,
    style::{Print, PrintStyledContent, Stylize},
};
use std::{env, io::stdout};
use which::which;

pub struct FetchData {
    pub label: &'static str,
    text: String,
}

impl FetchData {
    pub fn get(data: &FetchType, conf: &Config) -> Self {
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
                        let mut timestr = Vec::new();
                        if time >= 3600 {
                            timestr.push(format!("{}h", time / 3600));
                        }
                        if time % 3600 > 0 {
                            timestr.push(format!("{}m", (time % 3600) / 60));
                        }
                        if !timestr.is_empty() {
                            timestr.join(" ")
                        } else {
                            "0m".to_string()
                        }
                    }
                    Err(_) => "unknown".to_string(),
                },
            },
            FetchType::Packages => FetchData {
                label: "packages",
                text: packages::get_packages(conf.display_package_manager),
            },
            FetchType::Wm => FetchData {
                label: "wm",
                text: {
                    let get_no_wmctrl = || match sys_info::os_type()
                        .unwrap_or_else(|_| "Unknown".to_string())
                        .as_str()
                    {
                        "Linux" => env::var("XDG_SESSION_DESKTOP")
                            .or_else(|_| env::var("DESKTOP_SESSION"))
                            .unwrap_or_else(|_| "unknown".to_string()),
                        // TODO: Handle windows/mac os
                        _ => "unknown".to_string(),
                    };

                    if conf.use_wmctrl && which("wmctrl").is_ok() {
                        let try_get_wmctrl = || -> Result<String, anyhow::Error> {
                            let out = std::process::Command::new("wmctrl").args(["-m"]).output()?;
                            let s = std::str::from_utf8(&out.stdout)?;
                            match s.lines().find(|x| x.starts_with("Name: ")) {
                                Some(line) => {
                                    Ok(line.strip_prefix("Name: ").unwrap_or("error").to_string())
                                }
                                None => Ok("unknown".to_string()),
                            }
                        };

                        match try_get_wmctrl() {
                            Ok(wm) => wm,
                            Err(_) => get_no_wmctrl(),
                        }
                    } else {
                        get_no_wmctrl()
                    }
                },
            },
            #[allow(unreachable_patterns)]
            _ => todo!(),
        }
    }

    pub fn queue_print(&self, data_pos: u16, ascii_padding: u16) {
        queue!(
            stdout(),
            MoveRight(ascii_padding),
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
    conf.data.iter().map(|d| FetchData::get(d, conf)).collect()
}

pub fn print_all_fetches(
    data: &[FetchData],
    conf: &Config,
    ascii_padding: u16,
    ascii_lines: u16,
) -> u16 {
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
    let ascii_space_diff = ascii_lines.saturating_sub(data.len().try_into().unwrap_or(u16::MAX));
    let data_top_padding = ascii_space_diff / 2;
    let data_bottom_padding = ascii_space_diff / 2 + (ascii_space_diff % 2);

    let print_newline = |_| queue!(stdout(), Print("\n"),).unwrap();
    (0..data_top_padding).for_each(&print_newline);
    for d in data {
        d.queue_print(max_label_len, ascii_padding);
    }
    (0..data_bottom_padding).for_each(&print_newline);
    data.len().try_into().unwrap_or(u16::MAX)
}
