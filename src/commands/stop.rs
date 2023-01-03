use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use anyhow::anyhow;
use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use diesel::OptionalExtension;
use aze::ago;
use aze::database::establish_connection;
use aze::models::Frame;

use super::MyCommand;

#[derive(clap::Args, Debug)]
pub struct StopSubcommand {
    pub at: Option<NaiveDateTime>,
}

impl MyCommand for StopSubcommand {
    fn run(&self, output: super::Output) -> Result<()> {
        use aze::schema::frames::dsl::*;

        let at = self.at;

        let now = Local::now().naive_local();
        let started_at: NaiveDateTime;
        if at.is_some() {
            started_at = at.unwrap();
        } else {
            started_at = now;
        }

        let mut conn = establish_connection();

        let result = frames
            .filter(deleted.eq(false))
            .filter(end.is_null())
            .order_by(start.desc())
            .first::<Frame>(&mut conn)
            .optional()
            .expect("Error loading frames");

        if result.is_none() {
            return Err(anyhow!("No project started."));
        }
        let frame = result.unwrap();
        let _result = diesel::update(&frame)
            .set(end.eq(started_at))
            .execute(&mut conn);

        writeln!(
            output.out,
            "Stopping project {}, started {} and stopped {}",
            frame.project,
            ago(frame.start),
            ago(started_at.to_owned())
        )?;
        Ok(())
    }
}
