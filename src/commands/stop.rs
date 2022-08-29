use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use diesel::OptionalExtension;
use mycroft::ago;
use mycroft::establish_connection;
use mycroft::models::Frame;

use super::MyCommand;

#[derive(clap::Args, Debug)]
pub struct StopSubcommand {
    pub at: Option<NaiveDateTime>,
}

impl MyCommand for StopSubcommand {
    fn run(&self) -> Result<()> {
        use mycroft::schema::frames::dsl::*;

        let at = self.at;

        let now = Local::now().naive_local();
        let started_at: NaiveDateTime;
        if at.is_some() {
            started_at = at.unwrap();
        } else {
            started_at = now;
        }

        let conn = establish_connection();

        let result = frames
            .filter(deleted.eq(false))
            .filter(end.is_null())
            .order_by(start.desc())
            .first::<Frame>(&conn)
            .optional()
            .expect("Error loading frames");

        if result.is_none() {
            println!("No project started.");
        } else {
            let frame = result.unwrap();
            let _result = diesel::update(&frame)
                .set(end.eq(started_at))
                .execute(&conn);

            println!(
                "Stopping project {}, started {} and stopped {}",
                frame.project,
                ago(frame.start),
                ago(started_at.to_owned())
            );
        }
        return Ok(());
    }
}
