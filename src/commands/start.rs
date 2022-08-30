use anyhow::anyhow;
use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use mycroft::cli::convert_tags;
use mycroft::cli::parse_to_datetime;
use mycroft::cli::process_project;
use mycroft::cli::process_tags;
use mycroft::database::establish_connection;
use mycroft::service::frame::frame_start_collides;
use mycroft::service::frame::last_finished_frame;
use mycroft::service::frame::last_started_frame;
use mycroft::service::frame::start_frame;

use super::MyCommand;

#[derive(clap::Args, Debug)]
pub struct StartSubcommand {
    pub project: String,

    #[clap(value_parser = convert_tags)]
    pub tags: Vec<String>,

    #[clap(long = "at", value_parser = parse_to_datetime)]
    pub at: Option<NaiveDateTime>,

    #[clap(short = 'c', long = "confirm-new-project")]
    pub confirm_project: bool,

    #[clap(short = 'b', long = "confirm-new-tags")]
    pub confirm_tags: bool,

    #[clap(name = "gap", short = 'g', long = "gap", conflicts_with = "no_gap")]
    pub gap: bool,

    #[clap(name = "no_gap", short = 'G', long = "no-gap", conflicts_with_all = &["gap", "at"])]
    pub no_gap: bool,
}

impl MyCommand for StartSubcommand {
    fn run(&self) -> Result<()> {
        let project_string = self.project.to_string();
        let at = self.at;
        let conn = establish_connection();

        let now = Local::now().naive_local();
        let started_at: NaiveDateTime;
        if at.is_some() {
            started_at = at.unwrap();
        } else if self.no_gap {
            let last_finished = last_finished_frame(&conn);
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

        let result = last_started_frame(&conn);
        if result.is_some() {
            return Err(anyhow!(format!(
                "Project {} is already started",
                result.unwrap().project
            )));
        }

        let mut confirm = process_project(self.project.to_string(), self.confirm_project);
        if !confirm {
            return Err(anyhow!("Aborted!"));
        }
        confirm = process_tags(self.tags.to_owned(), self.confirm_tags);
        if !confirm {
            return Err(anyhow!("Aborted!"));
        }
        println!(
            "starting project {}{} at {}",
            project_string,
            if self.tags.len() > 0 {
                format!(" [{}]", self.tags.join(", "))
            } else {
                "".to_string()
            },
            started_at.format("%d.%m.%Y %H:%M"),
        );

        start_frame(&conn, &started_at, &project_string, self.tags.to_owned());
        return Ok(());
    }
}
