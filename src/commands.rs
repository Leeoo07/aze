pub mod add;
pub mod log;
pub mod start;
pub mod status;
pub mod stop;

use anyhow::Result;

pub trait MyCommand {
    fn run(&self) -> Result<()>;
}
