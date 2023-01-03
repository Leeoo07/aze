use std::{env, path::{MAIN_SEPARATOR, Path}, fs::create_dir_all};

use anyhow::Result;
use confy::ConfyError;
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub data_dir: String,
    pub datetime_format: String,
}

impl AppConfig {
    pub fn database_url(&self) -> String {
        let path = Path::new(&self.data_dir);

        if !path.exists() {
            create_dir_all(path).expect(&format!("Failed to create data directory {}", self.data_dir));
        }
        let default = format!("{}{}{}", self.data_dir, MAIN_SEPARATOR, "frames.db");

        env::var("DATABASE_URL").unwrap_or(default)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        if let Some(proj_dirs) = ProjectDirs::from("ch", "lethani", "aze") {
            return AppConfig {
                data_dir: proj_dirs.data_dir().to_str().unwrap().to_string(),
                datetime_format: "%Y-%m-%d %H:%M".to_string(),
            };
        }

        panic!("Could not evaluate data_dir");
    }
}

pub fn load_config() -> AppConfig {
    let cfg: Result<AppConfig, ConfyError> = confy::load("aze");

    cfg.unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use directories_next::ProjectDirs;

    #[test]
    fn default_config_dir() {
        let config = super::load_config();

        if let Some(proj_dirs) = ProjectDirs::from("ch", "lethani", "aze") {
            assert_eq!(
                proj_dirs.data_dir().to_str().unwrap().to_string(),
                config.data_dir
            );
        } else {
            panic!("Could not evaluate directory");
        }
    }
}
