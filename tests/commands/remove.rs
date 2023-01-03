use assert_cmd::prelude::*;

use chrono::{Local, NaiveDateTime, Date, NaiveDate};
use predicates::prelude::*;
use std::{process::{Command, Stdio}, fs::File};
use std::io::Write;
use crate::{TestDb, commands::get_frames};
use std::io::Read;
use super::add_frame;

#[test]
fn remove_no_id_given() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("remove")
        .env("DATABASE_URL", "file::memory:?cache=shared");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("The following required arguments were not provided"));

    Ok(())
}

#[test]
fn remove_with_id_nonexistent() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("remove")
        .arg("aaaaaaa")
        .env("DATABASE_URL", "file::memory:?cache=shared");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No frame found with id aaaaaaa."));

    Ok(())
}


#[test]
fn remove_with_id() -> Result<(), Box<dyn std::error::Error>> {
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
    assert_eq!(false, result.get(0).expect("fail").deleted);

    let id = &result.get(0).expect("fail").id;

    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.env("DATABASE_URL", &database).stdin(Stdio::piped()).arg("remove").arg(id).arg("--force").unwrap();
    cmd.assert()
        .success();

    let result = get_frames(&test_db);
    assert_eq!(1, result.len());
    assert_eq!(true, result.get(0).expect("fail").deleted);

    Ok(())
}

