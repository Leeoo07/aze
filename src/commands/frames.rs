use anyhow::Result;
use mycroft::service::frame::find_all;

use super::MyCommand;

#[derive(clap::Args, Debug)]
#[clap(
    about = "Display the list of all frame IDs.",
    after_help = "Example:\n\n$ mycroft frames\nf1c4815\n9d1a989\n8801ec3"
)]
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
