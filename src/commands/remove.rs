use anyhow::anyhow;
use anyhow::Result;
use colored::Colorize;
use dialoguer::Confirm;
use mycroft::service::{project::find_all, frame::find_frame};

use super::MyCommand;

#[derive(clap::Args, Debug)]
#[clap(
    about = "Remove a frame. You can specify the frame either by id or by position (ex: `-1` for the last frame).",
)]
pub struct RemoveSubcommand {

    #[clap(
        help = "Frame id or position"
    )]
    pub id: String,

    #[clap(
        short = 'f',
        long = "force",
        display_order = 1,
        help = "Don't ask for confirmation."
    )]
    pub force: bool
}

impl MyCommand for RemoveSubcommand {
    fn run(&self, output: super::Output) -> Result<()> {

        let frame_un = find_frame(&self.id);

        if frame_un.is_err() {
            return Err(anyhow!("No frame found with id {}.", self.id));
        }

        let frame = frame_un.unwrap();
        write!(
            output.out,
            "You are about to remove frame {} from {}{}",
            frame.project.purple(),
            frame.start.format("d.m.Y H:M").to_string().green(),
            if frame.end.is_some() {
                format!(" to {}", frame.end.unwrap().format("d.m.Y H:M").to_string().green())
            } else {
                format!("")
            }
        );
        if !Confirm::new().with_prompt(", continue?",).interact()? {
            println!("Looks like you want to continue");
        }

        Ok(())
    }
}
