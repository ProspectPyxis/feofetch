use crate::args::Args;
use anyhow::Context;
use serde::Deserialize;
use std::{default::Default, fs, io::ErrorKind, path::PathBuf};
use termcolor::Color;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FetchType {
	Os,
	Version,
	Uptime,
	Packages,
	Wm,
	Shell,
	Terminal,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ColorConfig {
	Normal,
	Black,
	#[serde(alias = "dark_gray")]
	DarkGrey,
	Red,
	DarkRed,
	Green,
	DarkGreen,
	Yellow,
	DarkYellow,
	Blue,
	DarkBlue,
	Magenta,
	DarkMagenta,
	Cyan,
	DarkCyan,
	White,
	#[serde(alias = "gray")]
	Grey,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
	pub data: Vec<FetchType>,
	pub use_icons: bool,
	pub label_color: ColorConfig,
	pub align_spaces: usize,
	pub offset: (usize, usize),
	pub padding_lines: usize,
	pub ascii: AsciiConfig,
	pub packages: PackagesConfig,
	pub wm: WmConfig,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct AsciiConfig {
	pub print: bool,
	pub ascii_path: Option<String>,
	pub color: ColorConfig,
	pub align_spaces: usize,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct PackagesConfig {
	pub print_package_manager_names: bool,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct WmConfig {
	pub use_wmctrl: bool,
}

impl ColorConfig {
	pub fn get_color(&self) -> Option<Color> {
		match self {
			ColorConfig::Normal => None,
			ColorConfig::Black | ColorConfig::DarkGrey => Some(Color::Black),
			ColorConfig::Red | ColorConfig::DarkRed => Some(Color::Red),
			ColorConfig::Green | ColorConfig::DarkGreen => Some(Color::Green),
			ColorConfig::Yellow | ColorConfig::DarkYellow => Some(Color::Yellow),
			ColorConfig::Blue | ColorConfig::DarkBlue => Some(Color::Blue),
			ColorConfig::Magenta | ColorConfig::DarkMagenta => Some(Color::Magenta),
			ColorConfig::Cyan | ColorConfig::DarkCyan => Some(Color::Cyan),
			ColorConfig::White | ColorConfig::Grey => Some(Color::White),
		}
	}

	pub fn is_intense(&self) -> bool {
		let bright_colors = [
			ColorConfig::DarkGrey,
			ColorConfig::Red,
			ColorConfig::Green,
			ColorConfig::Yellow,
			ColorConfig::Blue,
			ColorConfig::Magenta,
			ColorConfig::Cyan,
			ColorConfig::White,
		];

		bright_colors.contains(self)
	}
}

impl Default for Config {
	fn default() -> Self {
		Config {
			data: vec![
				FetchType::Os,
				FetchType::Version,
				FetchType::Uptime,
				FetchType::Packages,
				FetchType::Wm,
			],
			use_icons: false,
			label_color: ColorConfig::Cyan,
			align_spaces: 2,
			offset: (0, 0),
			padding_lines: 1,
			ascii: AsciiConfig::default(),
			packages: PackagesConfig::default(),
			wm: WmConfig::default(),
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
			padding_lines: args.padding_lines.unwrap_or(self.padding_lines),
			..self
		}
	}
}

impl Default for AsciiConfig {
	fn default() -> Self {
		AsciiConfig {
			print: false,
			ascii_path: None,
			color: ColorConfig::Normal,
			align_spaces: 2,
		}
	}
}

pub fn get_config(config_path: &PathBuf) -> anyhow::Result<Config> {
	let config = match fs::read_to_string(config_path) {
		Ok(config_str) => toml::from_str(&config_str).with_context(|| {
			format!(
				"Failed to parse config TOML at {}",
				config_path.to_string_lossy()
			)
		})?,
		Err(e) => match e.kind() {
			ErrorKind::NotFound => Config::default(),
			_ => {
				return Err(e).with_context(|| {
					format!(
						"Failed to read config TOML from {}",
						config_path.to_string_lossy()
					)
				})
			}
		},
	};
	Ok(config)
}
