use anyhow::Context;
use clap::Parser;
use serde::Deserialize;
use std::{default::Default, fs, io::ErrorKind, path::PathBuf};

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
	/// Override default config location with the specified path
	#[arg(short, long)]
	pub config_path: Option<String>,
	/// Set x-offset to this number, overriding config
	#[arg(short, long)]
	pub x_offset: Option<usize>,
	/// Set y-offset to this number, overriding config
	#[arg(short, long)]
	pub y_offset: Option<usize>,
	/// Set padding lines to this number, overriding config
	#[arg(short, long)]
	pub padding_lines: Option<usize>,
}

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

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
	pub use_icons: bool,
	pub data: Vec<FetchType>,
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
