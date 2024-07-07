use home::home_dir;
use std::fs;

pub struct Configuration {}

impl Configuration {
    pub fn new(use_config: bool) -> Self {
        if !use_config {
            return Self {};
        }

        let config_file_path = Self::get_config_file_path();

        if !fs::metadata(&config_file_path).is_ok() {
            fs::write(&config_file_path, "")
                .expect("Unable to create config file. If this is intentional, consider running with --no-config");
        }

        Self {}
    }

    #[cfg(target_family = "windows")]
    fn get_config_file_path() -> String {
        let home = home_dir().expect("Unable to get home directory");
        let config_dir = home.join(".rustfetch");

        if !config_dir.exists() {
            fs::create_dir(&config_dir).expect("Unable to create config directory");
        }

        return home_dir()
            .unwrap()
            .join(".rustfetch/config.toml")
            .to_str()
            .unwrap()
            .to_string();
    }

    #[cfg(target_family = "unix")]
    fn get_config_file_path() -> String {
        let home = dirs::home_dir().expect("Unable to get home directory");
        let config_dir = home.join(".rustfetch");

        if !config_dir.exists() {
            fs::create_dir(&config_dir).expect("Unable to create config directory");
        }

        return "~/.rustfetch/config.toml".to_string();
    }
}
