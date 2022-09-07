use assert_cmd::prelude::*;
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use std::process::Command;

use crate::{commands::add_frame, TestDb};

#[test]
fn time_from_cannot_be_parsed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", ":memory:")
        .arg("log")
        .arg("--from")
        .arg("test1");

    cmd.assert().failure().stderr(predicates::str::contains(
        "Invalid value \"test1\" for '--from <FROM>'",
    ));

    Ok(())
}

#[test]
fn time_can_be_parsed_by_date() -> Result<(), Box<dyn std::error::Error>> {
    let today = NaiveDate::pred(&Local::today().naive_local());

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", "file::memory:?cache=shared")
        .arg("log")
        .arg("--from")
        .arg(format!(
            "{}-{}-{}",
            today.year(),
            today.month(),
            today.day()
        ));

    cmd.assert().success().stdout(predicates::str::is_empty());

    Ok(())
}

#[test]
fn time_can_be_parsed_by_datetime() -> Result<(), Box<dyn std::error::Error>> {
    let today = NaiveDate::pred(&Local::today().naive_local());

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", "file::memory:?cache=shared")
        .arg("log")
        .arg("--from")
        .arg(format!(
            "{}-{}-{} 00:00",
            today.year(),
            today.month(),
            today.day(),
        ));

    cmd.assert().success().stdout(predicates::str::is_empty());

    Ok(())
}

#[test]
fn time_from_cannot_past_to() -> Result<(), Box<dyn std::error::Error>> {
    let day_after_tomorrow = NaiveDate::succ(&Local::today().naive_local()).succ();

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", "file::memory:?cache=shared")
        .arg("log")
        .arg("--from")
        .arg(format!(
            "{}-{}-{}",
            day_after_tomorrow.year(),
            day_after_tomorrow.month(),
            day_after_tomorrow.day()
        ));

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("'from' must be anterior to 'to'"));

    Ok(())
}

#[test]
fn time_to_is_inclusive_tomorrow_default() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = Local::now().naive_local().timestamp() + 7200;
    let end = start + 1800;

    add_frame(
        &test_db,
        &"test",
        &NaiveDateTime::from_timestamp(start, 0),
        Some(&NaiveDateTime::from_timestamp(end, 0)),
        None,
    )?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database).arg("log");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("test"));

    Ok(())
}

#[test]
fn time_from_searches_for_start() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = NaiveDate::from_ymd(2000, 1, 1).and_hms(12, 0, 0);
    let end = NaiveDate::from_ymd(2000, 1, 1).and_hms(13, 0, 0);

    add_frame(&test_db, &"test", &start, Some(&end), None)?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("--from")
        .arg("2000-01-01 12:30");

    cmd.assert().success().stdout(predicates::str::is_empty());

    Ok(())
}

#[test]
fn time_from_includes() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = NaiveDate::from_ymd(2000, 1, 1).and_hms(12, 0, 0);
    let end = NaiveDate::from_ymd(2000, 1, 1).and_hms(13, 0, 0);

    add_frame(&test_db, &"test", &start, Some(&end), None)?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("--from")
        .arg("2000-01-01 11:00");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("test"));

    Ok(())
}

#[test]
fn time_to_includes() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = NaiveDate::from_ymd(2000, 1, 1).and_hms(12, 0, 0);
    let end = NaiveDate::from_ymd(2000, 1, 1).and_hms(13, 0, 0);

    add_frame(&test_db, &"test", &start, Some(&end), None)?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("--from")
        .arg("2000-01-01 11:00")
        .arg("--to")
        .arg("2000-01-01 14:00");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("test"));

    Ok(())
}

#[test]
fn time_to_searches_for_end() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = NaiveDate::from_ymd(2000, 1, 1).and_hms(12, 0, 0);
    let end = NaiveDate::from_ymd(2000, 1, 1).and_hms(13, 0, 0);

    add_frame(&test_db, &"test", &start, Some(&end), None)?;

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("--from")
        .arg("2000-01-01 11:00")
        .arg("--to")
        .arg("2000-01-01 12:30");

    cmd.assert().success().stdout(predicates::str::is_empty());

    Ok(())
}
