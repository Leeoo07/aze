use anyhow::Result;
use colored::Colorize;
use aze::service::project::find_all;

use super::MyCommand;

#[derive(clap::Args, Debug)]
#[clap(
    about = "Display the list of all the existing projectst.",
    after_help = "Example:\n\n$ aze projects\napollo11\nhubble\nvoyager1\nvoyager2"
)]
pub struct ProjectsSubcommand {

}

impl MyCommand for ProjectsSubcommand {
    fn run(&self, output: super::Output) -> Result<()> {
        let projects = find_all();

        for project in projects {
            writeln!(output.out, "{}", project.purple())?;
        }


        Ok(())
    }
}
