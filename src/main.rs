extern crate diesel;

use chrono::{format::ParseError, NaiveDateTime};
use chrono::{Datelike, Duration, Local, NaiveDate};
use clap::{Arg, ArgAction, Command};

use anyhow::Result;
use mycroft::{create_frame, establish_connection, models::Frame, start_frame};

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::display::Display;

mod config;
mod display;

fn main() -> Result<()> {
    use mycroft::schema::frames::dsl::*;

    let matches = Command::new("mycroft")
        .about("Mycroft is a tool aimed at helping you monitoring time.")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("kreemer")
        .subcommand(
            Command::new("add")
                .about("Add time to a project with tag(s) that was not tracked live.")
                .arg(
                    Arg::new("from")
                        .short('f')
                        .long("from")
                        .help("Date and time of start of tracked activity")
                        .value_parser(parse_to_datetime)
                        .action(ArgAction::Set)
                        .required(true),
                )
                .arg(
                    Arg::new("to")
                        .long("to")
                        .short('t')
                        .help("Date and time of end of tracked activity")
                        .value_parser(parse_to_datetime)
                        .action(ArgAction::Set)
                        .required(true),
                )
                .arg(
                    Arg::new("project")
                        .help("Name of the project")
                        .action(ArgAction::Set)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("start").about("Start new activity.")
            .arg(
                Arg::new("project")
                    .action(ArgAction::Set)
                    .help("Name of the project")
            )
            .arg(
                Arg::new("at")
                    .long("at")
                    .action(ArgAction::Set)
                    .value_parser(parse_to_datetime)
                    .help("When did the frame start")
                    .required(false),

            ),
        )
        .subcommand(
            Command::new("stop").about("Stop the last activity.")
            .arg(
                Arg::new("at")
                    .long("at")
                    .action(ArgAction::Set)
                    .value_parser(parse_to_datetime)
                    .help("When did the activity stop")
                    .required(false),

            ),
        )
        .subcommand(
            Command::new("log").about("Display each recorded session during the given timespan.").arg(
                Arg::new("project")
                    .short('p')
                    .long("project")
                    .action(ArgAction::Set)
                    .help("Logs activity only for the given project. You can add other projects by using this option several times.")
            )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", command_matches)) => {
            let f = command_matches
                .get_one::<NaiveDateTime>("from")
                .expect("required");
            let t = command_matches
                .get_one::<NaiveDateTime>("to")
                .expect("required");
            let p = command_matches
                .get_one::<String>("project")
                .expect("required");

            println!(
                "starting project {} from {} to {}",
                p,
                f.format("%d.%m.%Y %H:%M"),
                t.format("%d.%m.%Y %H:%M")
            );

            let conn = establish_connection();
            create_frame(&conn, f, t, p);
        }
        Some(("start", command_matches)) => {
            let p = command_matches
                .get_one::<String>("project")
                .expect("required");

            let at = command_matches.get_one::<NaiveDateTime>("at");

            let now = Local::now().naive_local();
            let started_at: &NaiveDateTime;
            if at.is_some() {
                started_at = at.unwrap();
            } else {
                started_at = &now;
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
                    p,
                    started_at.format("%d.%m.%Y %H:%M"),
                );
                start_frame(&conn, started_at, p);
            }
        }
        Some(("log", command_matches)) => {
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
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }

    Ok(())
}

fn parse_to_datetime(s: &str) -> Result<NaiveDateTime, ParseError> {
    return NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S");
}
