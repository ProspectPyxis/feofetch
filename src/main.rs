mod ascii;
mod config;
mod fetch;
mod packages;

use etcetera::app_strategy::{self, AppStrategy, AppStrategyArgs};

fn main() {
    let strategy = app_strategy::choose_app_strategy(AppStrategyArgs {
        top_level_domain: "com".to_string(),
        author: "ProspectPyxis".to_string(),
        app_name: "feofetch".to_string(),
    })
    .unwrap();

    let conf = config::get_config(strategy.in_config_dir("config.toml"));
    let ascii_padding = if conf.print_ascii {
        match conf.ascii_path {
            Some(path) => ascii::print_ascii(path),
            None => panic!("print_ascii is true, but no ascii_path was specified"),
        }
    } else {
        0
    };

    let data = fetch::fetch_all(&conf);
    fetch::print_all_fetches(&data, &conf);
}
