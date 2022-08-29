use anyhow::Result;

use super::MyCommand;

#[derive(clap::Args, Debug)]
pub struct AddSubcommand {
    pub project: String,
}

impl MyCommand for AddSubcommand {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
