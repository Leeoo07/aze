use assert_cmd::prelude::*;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Timelike};
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;

mod add;
mod log;

pub fn add_frame(
    database: &str,
    project: &str,
    from: &NaiveDateTime,
    to: &NaiveDateTime,
) -> Result<(), Box<dyn std::error::Error>> {
    add_frame_with_tags(database, project, from, to, vec![])
}

pub fn add_frame_with_tags(
    database: &str,
    project: &str,
    from: &NaiveDateTime,
    to: &NaiveDateTime,
    tags: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add")
        .env("DATABASE_URL", database)
        .arg("--from")
        .arg(from.format("%Y-%m-%d %H:%M").to_string())
        .arg("--to")
        .arg(to.format("%Y-%m-%d %H:%M").to_string())
        .arg(project);

    for tag in tags {
        cmd.arg(format!("+{}", tag));
    }

    cmd.assert().success();

    Ok(())
}
