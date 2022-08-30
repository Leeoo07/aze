pub mod add;
pub mod log;
pub mod start;
pub mod status;
pub mod stop;

use anyhow::Result;

use crate::config::{load_config, AppConfig};

pub trait MyCommand {
    fn run(&self) -> Result<()>;

    fn config(&self) -> AppConfig {
        return load_config();
    }
}
