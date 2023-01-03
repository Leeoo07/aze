use anyhow::anyhow;
use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use colored::Colorize;
use aze::cli::convert_tags;
use aze::cli::parse_to_datetime;
use aze::cli::process_project;
use aze::cli::process_tags;
use aze::service::frame::frame_start_collides;
use aze::service::frame::last_finished_frame;
use aze::service::frame::last_started_frame;
use aze::service::frame::start_frame;

use super::MyCommand;

#[derive(clap::Args, Debug)]
#[clap(
    about = "Start monitoring time for the given project.",
    after_help = "Example:\n\n$ aze start apollo11 +module +brakes --no-gap\nStarting project apollo11 [module, brakes] at 16:34"
)]
pub struct StartSubcommand {
    #[clap(help = "Name of the project which should be used to add time.")]
    pub project: String,

    #[clap(help = "Tag(s) which should be added to the activity. Each tag has to be prepended with a plus sign.", value_parser = convert_tags)]
    pub tags: Vec<String>,

    #[clap(help = "Start frame at this time.", display_order = 1, long = "at", value_parser = parse_to_datetime)]
    pub at: Option<NaiveDateTime>,

    #[clap(
        short = 'c',
        display_order = 4,
        help = "Confirm addition of new project.",
        long = "confirm-new-project"
    )]
    pub confirm_project: bool,

    #[clap(
        short = 'b',
        display_order = 5,
        help = "Confirm addition of new tag.",
        long = "confirm-new-tags"
    )]
    pub confirm_tags: bool,

    #[clap(
        name = "no_gap", 
        help = "Don't leave gap between end time of previous project and start time of the current.", 
        short = 'G', 
        long = "no-gap",
        display_order = 3,
        conflicts_with_all = &["at"]
    )]
    pub no_gap: bool,
}

impl MyCommand for StartSubcommand {
    fn run(&self, output: super::Output) -> Result<()> {
        let project_string = self.project.to_string();
        let at = self.at;

        let now = Local::now().naive_local();
        let started_at: NaiveDateTime;
        if at.is_some() {
            started_at = at.unwrap();
            // TODO: check if at is in the future
        } else if self.no_gap {
            let last_finished = last_finished_frame();
            if last_finished.is_none() {
                return Err(anyhow!("No finished frame found, 'no-gap' is not possible"));
            }
            started_at = last_finished.unwrap().end.unwrap();
        } else {
            started_at = now;
        }

        if frame_start_collides(&started_at) {
            return Err(anyhow!("Start collides with existing frame"));
        }

        let result = last_started_frame();
        if result.is_some() {
            return Err(anyhow!(format!(
                "Project {} is already started",
                result.unwrap().project
            )));
        }

        if !process_project(self.project.to_string(), self.confirm_project) {
            return Err(anyhow!("Aborted!"));
        }
        if !process_tags(self.tags.to_owned(), self.confirm_tags) {
            return Err(anyhow!("Aborted!"));
        }

        writeln!(
            output.out,
            "starting project {}{} at {}",
            project_string.purple(),
            if !self.tags.is_empty() {
                format!(" [{}]", self.tags.join(", ").blue())
            } else {
                "".to_string()
            },
            started_at.format("%d.%m.%Y %H:%M").to_string().cyan(),
        )?;

        start_frame(&started_at, &project_string, self.tags.to_owned());
        Ok(())
    }
}
