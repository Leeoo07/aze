use anyhow::Result;
use mycroft::service::frame::find_all;

use super::MyCommand;

#[derive(clap::Args, Debug)]
#[clap(about = "Edit a frame.")]
pub struct FramesSubcommand {

}

impl MyCommand for FramesSubcommand {
    fn run(&self, output: super::Output) -> Result<()> {
        let frames = find_all();

        for frame in frames {
            writeln!(
                output.out,
                "{}",
                &frame.id[..7].to_string()
            )?;
        }

        Ok(())
    }
}
