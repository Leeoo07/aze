use anyhow::Result;
use chrono::Datelike;
use chrono::Duration;
use chrono::Local;
use chrono::NaiveDate;
use mycroft::database::establish_connection;
use mycroft::display::Display;
use mycroft::models::Frame;

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use super::MyCommand;

#[derive(clap::Args, Debug)]
pub struct LogSubcommand {}

impl MyCommand for LogSubcommand {
    fn run(&self) -> Result<()> {
        use mycroft::schema::frames::dsl::*;

        let conn = establish_connection();

        let last_week = (Local::now() - Duration::weeks(1)).naive_utc();
        let results = frames
            .filter(deleted.eq(false))
            .filter(start.gt(last_week))
            .order_by(start.desc())
            .load::<Frame>(&conn)
            .expect("Error loading frames");

        let mut actual_day: Option<NaiveDate> = None;
        let mut list: Vec<Display> = Vec::new();

        for frame in results {
            let cloned_start = frame.start.date().clone();
            if actual_day.is_none() || actual_day.unwrap() != frame.start.date() {
                list.push(Display::new(cloned_start, vec![frame]));
                actual_day = Some(cloned_start);
            } else {
                list.last_mut().unwrap().add_frame(frame);
            }
        }

        for mut display in list {
            let duration = display.total_duration();

            println!(
                "{} {} {} {} ({}h {}m {}s)",
                display.date.weekday(),
                display.date.day(),
                display.date.month(),
                display.date.year(),
                duration.num_hours(),
                duration.num_minutes() - (duration.num_hours() * 60),
                duration.num_seconds() - (duration.num_minutes() * 60)
            );
            for frame in display.frames {
                if frame.end.is_none() {
                    continue;
                }
                let frame_duration = frame.end.unwrap() - frame.start;
                println!(
                    "\t{}\t{} to {}\t{}h {}m {}s\t{}",
                    &frame.id[..7],
                    frame.start.format("%H:%M"),
                    frame.end.unwrap().format("%H:%M"),
                    frame_duration.num_hours(),
                    frame_duration.num_minutes() - (frame_duration.num_hours() * 60),
                    frame_duration.num_seconds() - (frame_duration.num_minutes() * 60),
                    frame.project
                );
            }
        }

        return Ok(());
    }
}
