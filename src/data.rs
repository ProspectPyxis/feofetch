use crate::{
	config::{self, Config, FetchType},
	packages,
};
use crossterm::{
	queue,
	style::{Print, PrintStyledContent, Stylize},
};
use std::{
	env,
	io::{self, Stdout},
};
use which::which;

pub struct FetchData {
	pub label: &'static str,
	pub icon: &'static str,
	text: String,
}

impl FetchData {
	pub fn get(data: &FetchType, conf: &Config) -> Self {
		match data {
			FetchType::Os => FetchData {
				label: "os",
				icon: "",
				text: config::get_os(),
			},

			FetchType::Version => FetchData {
				label: "version",
				icon: "",
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
				icon: "",
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
				icon: "",
				text: packages::get_packages(conf.packages.print_package_manager_names),
			},

			FetchType::Wm => FetchData {
				label: "wm",
				icon: "",
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

					if conf.wm.use_wmctrl && which("wmctrl").is_ok() {
						let try_get_wmctrl = || -> Result<String, ()> {
							let out = std::process::Command::new("wmctrl")
								.arg("-m")
								.output()
								.map_err(|_| ())?;

							let s = std::str::from_utf8(&out.stdout).map_err(|_| ())?;

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
		}
	}

	pub fn queue_print(
		&self,
		stdout: &mut Stdout,
		data_pos: usize,
		conf: &Config,
	) -> Result<(), io::Error> {
		let label_text = if conf.use_icons {
			self.icon
		} else {
			self.label
		};

		queue!(
			stdout,
			PrintStyledContent(format!("{:data_pos$}", label_text).bold().cyan()),
			Print(&self.text),
		)
	}
}

pub fn fetch_all(conf: &Config) -> Vec<FetchData> {
	conf.data.iter().map(|d| FetchData::get(d, conf)).collect()
}
