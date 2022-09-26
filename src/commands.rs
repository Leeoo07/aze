pub mod add;
pub mod edit;
pub mod log;
pub mod start;
pub mod status;
pub mod stop;
pub mod frames;

use std::io::Write;

use anyhow::Result;

use crate::config::{load_config, AppConfig};

pub struct Output<'a> {
    pub out: &'a mut dyn Write,
}

pub trait MyCommand {
    fn run(&self, output: Output) -> Result<()>;

    fn config(&self) -> AppConfig {
        load_config()
    }
}
