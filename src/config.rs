use etcetera::app_strategy::{self, AppStrategy, AppStrategyArgs};
use serde::Deserialize;
use std::{default::Default, fs::File, io::Read};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum FetchType {
    Os,
    Version,
    Uptime,
    Packages,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub use_icons: bool,
    pub data: Vec<FetchType>,
    pub align_spaces: u16,
    pub display_package_manager: bool,
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
            ],
            align_spaces: 2,
            display_package_manager: false,
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

pub fn get_config() -> Config {
    let strategy = app_strategy::choose_app_strategy(AppStrategyArgs {
        top_level_domain: "com".to_string(),
        author: "ProspectPyxis".to_string(),
        app_name: "feofetch".to_string(),
    })
    .unwrap();

    let config_path = strategy.in_config_dir("config.toml");
    let config_file = File::open(config_path);
    match config_file {
        Ok(mut f) => {
            let mut contents = String::new();
            if let Err(e) = f.read_to_string(&mut contents) {
                eprintln!("Error reading config file: {}", e);
                eprintln!("Using default configuration instead");
                return Config::default();
            }
            match toml::from_str(&contents) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error parsing TOML config: {}", e);
                    eprintln!("Using default configuration instead");
                    Config::default()
                }
            }
        }
        Err(_) => Config::default(),
    }
}
