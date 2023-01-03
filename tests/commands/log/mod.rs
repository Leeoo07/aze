use assert_cmd::prelude::*;
use chrono::{Local, NaiveDateTime};
use predicates::prelude::*;
use regex::Regex;
use std::process::Command;
use tempfile::tempdir;

use crate::TestDb;

use super::add_frame;

mod project;
mod tags;
mod time_filter;

#[test]
fn nothing_if_no_entries() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let mut cmd = Command::cargo_bin("aze")?;
    cmd.arg("log").env("DATABASE_URL", &database);

    cmd.assert().success().stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn entry_from_this_day() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;

    let start = Local::now().naive_local().timestamp() - 3600;
    let end = start + 1800;

    let dt_start = NaiveDateTime::from_timestamp(start, 0);
    let dt_end = NaiveDateTime::from_timestamp(end, 0);

    add_frame(&test_db, &"test", &dt_start, Some(&dt_end), None)?;

    let mut cmd = Command::cargo_bin("aze")?;
    cmd.arg("log").env("DATABASE_URL", &database);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0h 30m 00s"));

    Ok(())
}

#[test]
fn entry_from_last_two_weeks_default_not_shown() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;

    let start = Local::now().naive_local().timestamp() - 3600 * 24 * 10;
    let end = start + 1800;

    let dt_start = NaiveDateTime::from_timestamp(start, 0);
    let dt_end = NaiveDateTime::from_timestamp(end, 0);

    add_frame(&test_db, &"test", &dt_start, Some(&dt_end), None)?;

    let mut cmd = Command::cargo_bin("aze")?;
    cmd.arg("log").env("DATABASE_URL", &database);

    cmd.assert().success().stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn duration_correctly_calculated() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &test_db,
        &"test",
        &NaiveDateTime::from_timestamp(start, 0),
        Some(&NaiveDateTime::from_timestamp(end, 0)),
        None,
    )?;
    add_frame(
        &test_db,
        &"test",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        Some(&NaiveDateTime::from_timestamp(end + 3600, 0)),
        None,
    )?;

    let mut cmd = Command::cargo_bin("aze")?;
    cmd.arg("log").env("DATABASE_URL", &database);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0h 30m 00s"))
        .stdout(predicate::str::contains("1h 00m 00s"));

    Ok(())
}

#[test]
fn entries_does_not_cover_current_frame() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = Local::now().naive_local().timestamp() - 7200;

    add_frame(
        &test_db,
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        None,
        None,
    )?;

    let mut cmd = Command::cargo_bin("aze")?;
    cmd.env("DATABASE_URL", &database).arg("log");

    cmd.assert().success().stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn entries_cover_current_frame_if_requested() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = Local::now().naive_local().timestamp() - 7200;

    add_frame(
        &test_db,
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        None,
        None,
    )?;

    let mut cmd = Command::cargo_bin("aze")?;
    cmd.env("DATABASE_URL", &database).arg("log").arg("-c");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"));

    Ok(())
}

#[test]
fn order_newest_top() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &test_db,
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        Some(&NaiveDateTime::from_timestamp(end, 0)),
        None,
    )?;
    add_frame(
        &test_db,
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        Some(&NaiveDateTime::from_timestamp(end + 3600, 0)),
        None,
    )?;

    let mut cmd = Command::cargo_bin("aze")?;
    cmd.env("DATABASE_URL", &database).arg("log").arg("-c");

    cmd.assert().success();
    let stdout = String::from_utf8(cmd.output().expect("err").stdout)
        .expect("err")
        .replace("\n", "");

    let re = Regex::new(r"^.*test2.*test1$").unwrap();
    assert!(re.is_match(stdout.as_str()), "Output: {}", stdout);
    Ok(())
}

#[test]
fn order_oldest_top() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &test_db,
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        Some(&NaiveDateTime::from_timestamp(end, 0)),
        None,
    )?;
    add_frame(
        &test_db,
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        Some(&NaiveDateTime::from_timestamp(end + 3600, 0)),
        None,
    )?;

    let mut cmd = Command::cargo_bin("aze")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-c")
        .arg("-r");

    cmd.assert().success();
    let stdout = String::from_utf8(cmd.output().expect("err").stdout)
        .expect("err")
        .replace("\n", "");

    let re = Regex::new(r"^.*test1.*test2$").unwrap();
    assert!(re.is_match(stdout.as_str()), "Output: {}", stdout);
    Ok(())
}
