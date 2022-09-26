use assert_cmd::prelude::*;

use chrono::{Local, NaiveDateTime, Date, NaiveDate};
use predicates::prelude::*;
use std::{process::{Command, Stdio}, fs::File};

use crate::{TestDb, commands::get_frames};
use std::io::Read;
use super::add_frame;

#[test]
fn frames_no_frames_saved() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("frames")
        .env("DATABASE_URL", "file::memory:?cache=shared");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn frames_frame_saved() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;

    add_frame(
        &test_db,
        &"test",
        &NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        Option::from(&NaiveDate::from_ymd(2016, 7, 8).and_hms(10, 11, 12)),
        None
    )?;


    let result = get_frames(&test_db);
    assert_eq!(1, result.len());

    let id = &result.get(0).expect("fail").id;

    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.env("DATABASE_URL", &database).arg("frames");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&id[..7].to_string()));



    Ok(())
}
