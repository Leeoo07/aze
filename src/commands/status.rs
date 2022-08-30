use super::MyCommand;
use crate::config::load_config;
use anyhow::Result;
use colored::Colorize;
use mycroft::ago;
use mycroft::database::establish_connection;
use mycroft::service::frame::last_started_frame;

#[derive(clap::Args, Debug)]
#[clap(
    about = "Display when the current project and the time spent since.",
    after_help = "Example:\n\n$ mycroft status\nProject apollo11 [brakes] started seconds ago (2014-05-19 14:32:41+0100)"
)]
pub struct StatusSubcommand {
    #[clap(
        help = "only output project",
        display_order = 1,
        short = 'p',
        long = "project",
        conflicts_with_all = &["show-elapsed", "show-tags"]
    )]
    pub show_project: bool,

    #[clap(help = "only show tags", display_order = 2, short = 't', long = "tags", conflicts_with_all = &["show-elapsed", "show-project"])]
    pub show_tags: bool,

    #[clap(
        help = "only show time elapsed",
        display_order = 3,
        short = 'e',
        long = "elapsed",
        conflicts_with_all = &["show-tags", "show-project"]
    )]
    pub show_elapsed: bool,
}

impl MyCommand for StatusSubcommand {
    fn run(&self) -> Result<()> {
        let conn = establish_connection();
        let result = last_started_frame(&conn);

        if result.is_none() {
            println!("No project started.");
            return Ok(());
        }

        let frame = result.unwrap();

        if self.show_project {
            println!("{}", frame.project.purple());
            return Ok(());
        }
        if self.show_tags {
            println!("{}", frame.tags.values().join(", ").cyan());
            return Ok(());
        }
        if self.show_elapsed {
            println!("{}", ago(frame.start).green());
            return Ok(());
        }

        println!(
            "Project {}{} started {} ({})",
            frame.project.purple(),
            if frame.tags.values().len() > 0 {
                format!(" [{}]", frame.tags.values().join(", ").blue())
            } else {
                "".to_string()
            },
            ago(frame.start).green(),
            frame
                .start
                .format(&load_config().datetime_format)
                .to_string()
                .cyan()
        );

        return Ok(());
    }
}
