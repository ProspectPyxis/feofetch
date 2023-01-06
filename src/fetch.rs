use crate::{
	config::{Config, FetchType},
	packages,
};
use std::{
	env,
	io::{self, Write},
};
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

mod wm {
	use crate::config::Config;
	use std::env;
	use which::which;

	fn get_no_wmctrl() -> String {
		match &sys_info::os_type().unwrap_or_else(|_| "Unknown".to_string())[..] {
			"Linux" => env::var("XDG_SESSION_DESKTOP")
				.or_else(|_| env::var("DESKTOP_SESSION"))
				.unwrap_or_else(|_| "unknown".to_string()),
			// TODO: Handle windows/mac os
			_ => "unknown".to_string(),
		}
	}

	fn try_get_wmctrl() -> Option<String> {
		let out = std::process::Command::new("wmctrl")
			.arg("-m")
			.output()
			.ok()?;

		let output_str = std::str::from_utf8(&out.stdout).ok()?;

		Some(
			output_str
				.lines()
				.find(|line| line.starts_with("Name: "))
				.map(|line| line.strip_prefix("Name: ").unwrap_or("error").to_string())
				.unwrap_or_else(|| "unknown".to_string()),
		)
	}

	pub fn get(conf: &Config) -> String {
		if conf.wm.use_wmctrl && which("wmctrl").is_ok() {
			try_get_wmctrl().unwrap_or_else(get_no_wmctrl)
		} else {
			get_no_wmctrl()
		}
	}
}

pub struct FetchData {
	pub label: &'static str,
	pub icon: &'static str,
	text: String,
}

impl FetchData {
	pub fn get(data: &FetchType, conf: &Config) -> Self {
		match data {
			FetchType::Os => {
				let os = sys_info::os_type().unwrap_or_else(|_| "Unknown".to_string());
				let os_str = if &os[..] == "Linux" {
					sys_info::linux_os_release()
						.ok()
						.and_then(|info| info.id)
						.unwrap_or_else(|| "linux".to_string())
				} else {
					os.to_lowercase()
				};

				FetchData {
					label: "os",
					icon: match &os[..] {
						"Linux" => "",
						"Darwin" => "",
						"Windows" => "",
						_ => "",
					},
					text: os_str,
				}
			}

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
				text: wm::get(conf),
			},

			FetchType::Shell => FetchData {
				label: "shell",
				icon: "",
				text: {
					env::var("SHELL")
						.ok()
						.and_then(|shell| {
							std::path::PathBuf::from(&shell)
								.file_name()
								.map(|s| s.to_string_lossy().to_string())
						})
						.unwrap_or_else(|| "unknown".to_string())
				},
			},

			FetchType::Terminal => FetchData {
				label: "terminal",
				icon: "",
				text: {
					match env::var("TERM_PROGRAM") {
						Ok(term) => term
							.strip_suffix(".app")
							.map(|term| term.to_string())
							.unwrap_or(term),
						Err(_) => "unknown".to_string(),
					}
				},
			},
		}
	}

	pub fn queue_print(
		&self,
		stdout: &mut StandardStream,
		data_pos: usize,
		conf: &Config,
	) -> Result<(), io::Error> {
		stdout.set_color(
			ColorSpec::new()
				.set_fg(conf.label_color.get_color())
				.set_intense(conf.label_color.is_intense())
				.set_bold(true),
		)?;
		write!(
			stdout,
			"{:data_pos$}",
			if conf.use_icons {
				self.icon
			} else {
				self.label
			},
		)?;
		stdout.set_color(&ColorSpec::new())?;
		write!(stdout, "{}", self.text)?;
		Ok(())
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
	let data_start = ascii_lines_count.saturating_sub(data.len()) / 2;
	let ascii_start = data.len().saturating_sub(ascii_lines_count) / 2;
	let total_lines = ascii_lines_count.max(data.len());

	let data_pos = if !conf.use_icons {
		data.iter()
			.fold(0, |acc, x| acc.max(x.label.chars().count()))
	} else {
		1
	} + conf.align_spaces;

	let mut stdout = StandardStream::stdout(ColorChoice::Auto);

	write!(&mut stdout, "{}", "\n".repeat(conf.offset.1))?;

	let mut ascii_lines = ascii.unwrap_or("").lines();
	let mut data_lines = data.iter();
	for index in 0..total_lines {
		write!(&mut stdout, "{}", " ".repeat(conf.offset.0))?;

		if index >= ascii_start {
			match ascii_lines.next() {
				Some(line) => {
					stdout.set_color(
						ColorSpec::new()
							.set_fg(conf.ascii.color.get_color())
							.set_intense(conf.ascii.color.is_intense())
							.set_bold(true),
					)?;
					write!(&mut stdout, "{:ascii_max_length$}", line)
				}
				None => write!(&mut stdout, "{}", " ".repeat(ascii_max_length)),
			}
		} else {
			write!(&mut stdout, "{}", " ".repeat(ascii_max_length))
		}?;

		if index >= data_start {
			if let Some(line) = data_lines.next() {
				line.queue_print(&mut stdout, data_pos, conf)?;
			}
		}

		writeln!(&mut stdout)?;
	}

	write!(&mut stdout, "{}", "\n".repeat(conf.padding_lines))?;

	Ok(())
}
