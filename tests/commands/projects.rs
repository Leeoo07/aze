use assert_cmd::prelude::*;

use chrono::{Local, NaiveDateTime, Date, NaiveDate};
use predicates::prelude::*;
use std::{process::{Command, Stdio}, fs::File};

use crate::{TestDb, commands::get_frames};
use std::io::Read;
use super::add_frame;

#[test]
fn projects_no_frames_saved() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("projects")
        .env("DATABASE_URL", "file::memory:?cache=shared");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn projects_frame_saved() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;

    add_frame(
        &test_db,
        &"test",
        &NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        Option::from(&NaiveDate::from_ymd(2016, 7, 8).and_hms(10, 11, 12)),
        None
    )?;


    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.env("DATABASE_URL", &database).arg("projects");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test"));

    Ok(())
}

#[test]
fn projects_frames_saved() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;

    add_frame(
        &test_db,
        &"test",
        &NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        Option::from(&NaiveDate::from_ymd(2016, 7, 8).and_hms(10, 11, 12)),
        None
    )?;

    add_frame(
        &test_db,
        &"test2",
        &NaiveDate::from_ymd(2018, 7, 8).and_hms(9, 10, 11),
        Option::from(&NaiveDate::from_ymd(2018, 7, 8).and_hms(10, 11, 12)),
        None
    )?;


    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.env("DATABASE_URL", &database).arg("projects");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test2"));

    Ok(())
}
