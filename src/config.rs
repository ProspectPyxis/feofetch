use crate::command_line::Args;
use serde::Deserialize;
use std::{default::Default, fs, io::ErrorKind, path::PathBuf};

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FetchType {
	Os,
	Version,
	Uptime,
	Packages,
	Wm,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
	pub use_icons: bool,
	pub data: Vec<FetchType>,
	pub align_spaces: usize,
	pub use_wmctrl: bool,
	pub offset: (usize, usize),
	pub padding_lines: usize,
	pub ascii: AsciiConfig,
	pub packages: PackagesConfig,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct AsciiConfig {
	pub print: bool,
	pub ascii_path: Option<String>,
	pub align_spaces: usize,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct PackagesConfig {
	pub print_package_manager_names: bool,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			use_icons: false,
			data: vec![
				FetchType::Os,
				FetchType::Version,
				FetchType::Uptime,
				FetchType::Packages,
				FetchType::Wm,
			],
			align_spaces: 2,
			use_wmctrl: false,
			offset: (0, 0),
			padding_lines: 1,
			ascii: AsciiConfig::default(),
			packages: PackagesConfig::default(),
		}
	}
}

impl Config {
	pub fn with_overrides(self, args: &Args) -> Self {
		Config {
			offset: (
				args.x_offset.unwrap_or(self.offset.0),
				args.y_offset.unwrap_or(self.offset.1),
			),
			..self
		}
	}
}

impl Default for AsciiConfig {
	fn default() -> Self {
		AsciiConfig {
			print: false,
			ascii_path: None,
			align_spaces: 2,
		}
	}
}

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

pub fn get_config(config_path: PathBuf) -> Config {
	let throw_and_default = |msg, e| {
		eprintln!("{}: {}", msg, e);
		eprintln!("Falling back to default configuration");
		Config::default()
	};

	match fs::read_to_string(config_path) {
		Ok(config_str) => match toml::from_str(&config_str) {
			Ok(c) => c,
			Err(e) => throw_and_default("Unable to parse TOML config", e.to_string()),
		},
		Err(e) => match e.kind() {
			ErrorKind::NotFound => Config::default(),
			_ => throw_and_default("Unable to read config.toml", e.to_string()),
		},
	}
}

pub fn load_raw_ascii(ascii_path: PathBuf) -> String {
	fs::read_to_string(&ascii_path).unwrap_or_else(|e| {
		panic!(
			"Unable to read ascii file \"{}\": {}",
			ascii_path.display(),
			e
		)
	})
}
