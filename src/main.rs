mod command_line;
mod config;
mod fetch;
mod packages;

use clap::Parser;
use etcetera::app_strategy::{self, AppStrategy, AppStrategyArgs};

fn main() {
    let strategy = app_strategy::choose_app_strategy(AppStrategyArgs {
        top_level_domain: "com".to_string(),
        author: "ProspectPyxis".to_string(),
        app_name: "feofetch".to_string(),
    })
    .unwrap();

    let args = command_line::Args::parse();
    let conf = config::get_config(if let Some(path) = args.config_path {
        match std::fs::canonicalize(&path) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Error: Invalid config path {} ({})", path, e);
                return;
            }
        }
    } else {
        strategy.in_config_dir("config.toml")
    });
    let ascii = if conf.ascii.print {
        Some(config::load_raw_ascii(match conf.ascii.ascii_path {
            Some(ref path) => path.parse().unwrap(),
            None => strategy.in_config_dir("ascii.txt"),
        }))
    } else {
        None
    };

    let data = fetch::fetch_all(&conf);
    fetch::print_all_fetches(&data, &conf, ascii.as_deref()).expect("Unable to write to stdout");
}
