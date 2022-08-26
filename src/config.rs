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

#[allow(dead_code)]
pub fn load_config() -> Result<AppConfig> {
    let cfg: Result<AppConfig, ConfyError> = confy::load("mycroft");

    return Ok(cfg.unwrap());
}

#[cfg(test)]
mod tests {
    use directories::ProjectDirs;

    #[test]
    fn default_config_dir() {
        let config = super::load_config();

        if config.is_err() {
            panic!("config is not loadable");
        }

        if let Some(proj_dirs) = ProjectDirs::from("ch", "lethani", "mycroft") {
            assert_eq!(
                proj_dirs.data_dir().to_str().unwrap().to_string(),
                config.unwrap().data_dir
            );
        } else {
            panic!("Could not evaluate directory");
        }
    }
}
