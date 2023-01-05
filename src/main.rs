mod args;
mod config;
mod fetch;
mod packages;

use anyhow::Context;
use clap::Parser;
use etcetera::app_strategy::{self, AppStrategy, AppStrategyArgs};
use std::fs::{canonicalize, read_to_string};

fn main() -> anyhow::Result<()> {
	let strategy = app_strategy::choose_app_strategy(AppStrategyArgs {
		top_level_domain: "com".to_string(),
		author: "ProspectPyxis".to_string(),
		app_name: "feofetch".to_string(),
	})
	.context("Failed to set up app strategy")?;

	let args = args::Args::parse();
	let config_path = match args.config_path {
		Some(ref path) => {
			if path.eq_ignore_ascii_case("default") {
				strategy.in_config_dir("config.toml")
			} else {
				canonicalize(path)
					.with_context(|| format!("Failed to parse config TOML path {}", path))?
			}
		}
		None => strategy.in_config_dir("config.toml"),
	};
	let conf = config::get_config(&config_path)
		.with_context(|| {
			format!(
				"Failed to read config file {}",
				config_path.to_string_lossy(),
			)
		})?
		.with_overrides(&args);
	let ascii_file = match conf.ascii.ascii_path {
		Some(ref path) => path
			.parse()
			.with_context(|| format!("Failed to parse ascii path {}", path))?,
		None => strategy.in_config_dir("ascii.txt"),
	};
	let ascii = conf
		.ascii
		.print
		.then_some(read_to_string(&ascii_file).with_context(|| {
			format!(
				"Failed to read ascii file from {}",
				ascii_file.to_string_lossy()
			)
		})?);

	let data = fetch::fetch_all(&conf);
	fetch::print_all_fetches(&data, &conf, ascii.as_deref())
		.context("Failed to write to stdout")?;

	Ok(())
}
