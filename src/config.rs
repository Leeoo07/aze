use anyhow::Result;
use confy::ConfyError;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub data_dir: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        if let Some(proj_dirs) = ProjectDirs::from("ch", "lethani", "mycroft") {
            return AppConfig {
                data_dir: proj_dirs.data_dir().to_str().unwrap().to_string(),
            };
        }

        panic!("Could not evaluate data_dir");
    }
}

pub fn load_config() -> Result<AppConfig> {
    let cfg: Result<AppConfig, ConfyError> = confy::load("mycroft");

    return Ok(cfg.unwrap());
}
