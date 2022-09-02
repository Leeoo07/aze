use assert_cmd::prelude::*;

use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn add_without_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add")
        .env("DATABASE_URL", "file::memory:?cache=shared");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("arguments were not provided"));

    Ok(())
}

#[test]
fn add_frame_with_correct_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add")
        .env("DATABASE_URL", "file::memory:?cache=shared")
        .arg("--from")
        .arg("2000-01-01 12:00")
        .arg("--to")
        .arg("2000-01-01 13:00")
        .arg("test");
    cmd.assert().success().stdout(predicate::str::contains(
        "starting project test from 2000-01-01 12:00 to 2000-01-01 13:00",
    ));

    Ok(())
}

#[test]
fn start_in_overlapping_frame_returns_in_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add")
        .env("DATABASE_URL", &database)
        .arg("--from")
        .arg("2000-01-01 12:00")
        .arg("--to")
        .arg("2000-01-01 16:00")
        .arg("test");
    cmd.assert().success().stdout(predicate::str::contains(
        "starting project test from 2000-01-01 12:00 to 2000-01-01 16:00",
    ));

    cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add")
        .env("DATABASE_URL", &database)
        .arg("--from")
        .arg("2000-01-01 15:00")
        .arg("--to")
        .arg("2000-01-01 17:00")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("overlaps with start and end"));

    Ok(())
}

#[test]
fn end_in_overlapping_frame_returns_in_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add")
        .env("DATABASE_URL", &database)
        .arg("--from")
        .arg("2000-01-01 12:00")
        .arg("--to")
        .arg("2000-01-01 16:00")
        .arg("test");
    cmd.assert().success().stdout(predicate::str::contains(
        "starting project test from 2000-01-01 12:00 to 2000-01-01 16:00",
    ));

    cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add")
        .env("DATABASE_URL", &database)
        .arg("--from")
        .arg("2000-01-01 10:00")
        .arg("--to")
        .arg("2000-01-01 13:00")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("overlaps with start and end"));

    Ok(())
}

#[test]
fn start_and_end_in_overlapping_frame_returns_in_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let database = dir.path().join("database.db");

    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add")
        .env("DATABASE_URL", &database)
        .arg("--from")
        .arg("2000-01-01 12:00")
        .arg("--to")
        .arg("2000-01-01 16:00")
        .arg("test");
    cmd.assert().success().stdout(predicate::str::contains(
        "starting project test from 2000-01-01 12:00 to 2000-01-01 16:00",
    ));

    cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add")
        .env("DATABASE_URL", &database)
        .arg("--from")
        .arg("2000-01-01 13:00")
        .arg("--to")
        .arg("2000-01-01 14:00")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("overlaps with start and end"));

    Ok(())
}
