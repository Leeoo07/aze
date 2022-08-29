use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use mycroft::start_frame;
use mycroft::{establish_connection, models::Frame};

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use super::MyCommand;

#[derive(clap::Args, Debug)]
pub struct StartSubcommand {
    pub project: String,
    pub at: Option<NaiveDateTime>,
}

impl MyCommand for StartSubcommand {
    fn run(&self) -> Result<()> {
        use mycroft::schema::frames::dsl::*;
        let project_string = self.project.to_string();
        let at = self.at;

        let now = Local::now().naive_local();
        let started_at: NaiveDateTime;
        if at.is_some() {
            started_at = at.unwrap();
        } else {
            started_at = now;
        }

        let conn = establish_connection();

        let results = frames
            .filter(deleted.eq(false))
            .filter(end.is_null())
            .order_by(start.desc())
            .load::<Frame>(&conn)
            .expect("Error loading frames");

        if results.len() > 0 {
            let frame = results.get(0);
            if frame.is_none() {
                eprintln!("Error: Something went terribly wrong");
            } else {
                eprintln!(
                    "Error: Project {} is already started",
                    frame.unwrap().project
                );
            }
        } else {
            println!(
                "starting project {} at {}",
                project_string,
                started_at.format("%d.%m.%Y %H:%M"),
            );
            start_frame(&conn, &started_at, &project_string);
        }
        return Ok(());
    }
}
