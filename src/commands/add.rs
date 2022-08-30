use anyhow::anyhow;
use anyhow::Result;
use chrono::NaiveDateTime;
use dialoguer::{theme::ColorfulTheme, Confirm};
use mycroft::cli::parse_to_datetime;
use mycroft::cli::process_project;
use mycroft::cli::process_tags;
use mycroft::database::establish_connection;
use mycroft::service::frame::create_frame;
use mycroft::service::frame::frame_collides;
use mycroft::service::project::has_project;

use super::MyCommand;

#[derive(clap::Args, Debug)]
pub struct AddSubcommand {
    pub project: String,

    pub tags: Vec<String>,

    #[clap(short = 'f', long = "from", value_parser = parse_to_datetime)]
    pub from: NaiveDateTime,

    #[clap(short = 't', long = "to", value_parser = parse_to_datetime)]
    pub to: NaiveDateTime,

    #[clap(short = 'c', long = "confirm-new-project")]
    pub confirm_project: bool,

    #[clap(short = 'b', long = "confirm-new-tags")]
    pub confirm_tags: bool,
}

impl MyCommand for AddSubcommand {
    fn run(&self) -> Result<()> {
        if self.confirm_project && !has_project(self.project.to_string()) {
            if !Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "Project '{}' does not exist yet. Create it?",
                    self.project
                ))
                .default(false)
                .interact()
                .unwrap()
            {
                return Err(anyhow!("Aborted!"));
            }
        }

        process_tags(self.tags.to_owned(), self.confirm_tags);
        process_project(self.project.to_string(), self.confirm_project);

        if frame_collides(&self.from, &self.to) {
            return Err(anyhow!(
                "Frame already exist which overlaps with start and end"
            ));
        }

        if self.tags.len() > 0 {
            println!(
                "starting project {} [{}] from {} to {}",
                self.project,
                self.tags.join(","),
                self.from.format("%d.%m.%Y %H:%M"),
                self.to.format("%d.%m.%Y %H:%M")
            );
        } else {
            println!(
                "starting project {} from {} to {}",
                self.project,
                self.from.format("%d.%m.%Y %H:%M"),
                self.to.format("%d.%m.%Y %H:%M")
            );
        }

        let conn = establish_connection();
        create_frame(
            &conn,
            &self.from,
            &self.to,
            &self.project,
            self.tags.to_owned(),
        );

        return Ok(());
    }
}
