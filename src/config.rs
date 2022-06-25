use etcetera::app_strategy::{self, AppStrategy, AppStrategyArgs};
use serde::Deserialize;
use std::{default::Default, fs::File, io::Read};

#[derive(Deserialize)]
pub struct Config {
    use_icons: bool,
    data: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            use_icons: false,
            data: vec!["os".to_string(), "version".to_string()],
        }
    }
}

pub fn get_config() -> Config {
    let strategy = app_strategy::choose_app_strategy(AppStrategyArgs {
        top_level_domain: "com".to_string(),
        author: "ProspectPyxis".to_string(),
        app_name: "feofetch".to_string(),
    }).unwrap();

    let config_path = strategy.in_config_dir("config.toml");
    let config_file = File::open(config_path);
    match config_file {
        Ok(mut f) => {
            let mut contents = String::new();
            if let Err(e) = f.read_to_string(&mut contents) {
                panic!("Failed to read config file: {}", e);
            }
            toml::from_str(&contents).unwrap()
        }
        Err(_) => Config::default(),
    }
}
