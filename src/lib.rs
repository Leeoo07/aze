pub mod schema;
use crate::models::NewFrame;
pub mod models;
use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use clap::{ArgMatches, Command};
use config::load_config;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use uuid::Uuid;

mod config;

#[macro_use]
extern crate diesel;

pub fn establish_connection() -> SqliteConnection {
    let cfg = load_config();

    if cfg.is_err() {
        panic!("Config could not be loaded");
    }
    let path = cfg.unwrap().data_dir + &std::path::MAIN_SEPARATOR.to_string() + "frames.db";

    return SqliteConnection::establish(&path).expect(&format!("Error connecting to {}", &path));
}

pub fn create_frame(
    conn: &SqliteConnection,
    start: &NaiveDateTime,
    end: &NaiveDateTime,
    project: &str,
) {
    use schema::frames;

    let uuid: Uuid = Uuid::new_v4();
    let end_value: Option<&NaiveDateTime> = Option::Some(&end);
    let new_frame = NewFrame {
        id: &uuid.to_string(),
        start,
        end: end_value,
        last_update: &NaiveDateTime::default(),
        project,
        deleted: &false,
    };

    diesel::insert_into(frames::table)
        .values(&new_frame)
        .execute(conn)
        .expect("Error saving new post");
}

pub fn start_frame(conn: &SqliteConnection, start: &NaiveDateTime, project: &str) {
    use schema::frames;

    let uuid: Uuid = Uuid::new_v4();

    let end: Option<&NaiveDateTime> = Option::None;

    let new_frame = NewFrame {
        id: &uuid.to_string(),
        start,
        end,
        last_update: &NaiveDateTime::default(),
        project,
        deleted: &false,
    };

    diesel::insert_into(frames::table)
        .values(&new_frame)
        .execute(conn)
        .expect("Error saving new frame");
}

pub fn ago(ago: NaiveDateTime) -> String {
    let now = Local::now().naive_local();

    let duration = now - ago;

    if duration.num_days() != 0 {
        return format!("{} days ago", duration.num_days());
    } else if duration.num_hours() != 0 {
        return format!("{} hours ago", duration.num_hours());
    } else if duration.num_minutes() != 0 {
        return format!("{} minutes ago", duration.num_minutes());
    } else if duration.num_seconds() != 0 {
        return format!("{} seconds ago", duration.num_seconds());
    } else {
        return format!("just now");
    }
}

trait AppCommand {
    fn command(&self) -> Command;
    fn action(&self, arguments: &ArgMatches) -> Result<()>;
}
