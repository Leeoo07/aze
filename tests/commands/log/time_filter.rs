use assert_cmd::prelude::*;
use chrono::{Datelike, Local, NaiveDate};
use std::process::Command;

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
