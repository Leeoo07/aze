pub mod schema;

pub mod models;
use chrono::{Local, NaiveDateTime};

pub mod cli;
pub mod config;
pub mod database;
pub mod display;
pub mod service;

#[macro_use]
extern crate diesel;

pub fn ago(ago: NaiveDateTime) -> String {
    let now = Local::now().naive_local();

    let duration = now - ago;

    if duration.num_days() != 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() != 0 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_minutes() != 0 {
        format!("{} minutes ago", duration.num_minutes())
    } else if duration.num_seconds() != 0 {
        format!("{} seconds ago", duration.num_seconds())
    } else {
        "just now".to_string()
    }
}
