use super::MyCommand;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use anyhow::Result;
use diesel::OptionalExtension;
use mycroft::ago;
use mycroft::establish_connection;
use mycroft::models::Frame;

#[derive(clap::Args, Debug)]
pub struct StatusSubcommand {}

impl MyCommand for StatusSubcommand {
    fn run(&self) -> Result<()> {
        use mycroft::schema::frames::dsl::*;

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

            println!(
                "Project {} started {} ({})",
                frame.project,
                ago(frame.start),
                frame.start.format("%d.%m.%Y %H:%M")
            );
        }

        return Ok(());
    }
}
