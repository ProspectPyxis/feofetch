use crate::{
    config::{self, Config, FetchType},
    packages,
};
use crossterm::{
    queue,
    style::{Print, PrintStyledContent, Stylize},
    QueueableCommand,
};
use std::{
    env,
    io::{self, stdout, Stdout},
};
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

    pub fn queue_print(&self, data_pos: usize, stdout: &mut Stdout) -> Result<(), io::Error> {
        queue!(
            stdout,
            PrintStyledContent(format!("{:data_pos$}", self.label).bold().cyan()),
            Print(&self.text),
        )
    }
}

pub fn fetch_all(conf: &Config) -> Vec<FetchData> {
    conf.data.iter().map(|d| FetchData::get(d, conf)).collect()
}

pub fn print_all_fetches(
    data: &[FetchData],
    conf: &Config,
    ascii: Option<&str>,
) -> Result<(), io::Error> {
    let (ascii_lines_count, ascii_max_length) = match ascii {
        Some(a) => (
            a.lines().count(),
            a.lines().fold(0, |acc, x| acc.max(x.chars().count())) + conf.ascii.align_spaces,
        ),
        None => (0, 0),
    };
    let data_start = (ascii_lines_count.saturating_sub(data.len()) / 2).min(ascii_lines_count);
    let total_lines = ascii_lines_count.max(data.len());

    let data_pos = data
        .iter()
        .fold(0, |acc, x| acc.max(x.label.chars().count()))
        + conf.align_spaces;

    let mut stdout = stdout();

    for _ in 0..conf.offset.1 {
        stdout
            .queue(Print('\n'))?;
    }

    let mut ascii_lines = ascii.unwrap_or("").lines().peekable();
    let mut data_lines = data.iter().peekable();
    for index in 0..total_lines {
        stdout.queue(Print(" ".repeat(conf.offset.0)))?;

        if ascii_lines.peek().is_some() {
            stdout.queue(PrintStyledContent(
                format!("{:ascii_max_length$}", ascii_lines.next().unwrap()).bold(),
            ))?;
        } else {
            stdout.queue(Print(" ".repeat(ascii_max_length)))?;
        }

        if index >= data_start && data_lines.peek().is_some() {
            data_lines
                .next()
                .unwrap()
                .queue_print(data_pos, &mut stdout)?;
        }

        stdout.queue(Print('\n'))?;
    }

    Ok(())
}
