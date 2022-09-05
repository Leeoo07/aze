use assert_cmd::prelude::*;

use predicates::prelude::*;
use std::process::Command;

use crate::{commands::get_frames, TestDb};

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
    let test_db = TestDb::new();
    let database = &test_db.db_path;
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

    let result = get_frames(&test_db);
    assert_eq!(1, result.len());
    Ok(())
}

#[test]
fn end_in_overlapping_frame_returns_in_error() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
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

    let result = get_frames(&test_db);
    assert_eq!(1, result.len());
    Ok(())
}

#[test]
fn start_and_end_in_overlapping_frame_returns_in_error() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
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

    let result = get_frames(&test_db);
    assert_eq!(1, result.len());
    Ok(())
}

#[test]
fn adding_tags_to_frames() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add")
        .env("DATABASE_URL", &database)
        .arg("--from")
        .arg("2000-01-01 13:00")
        .arg("--to")
        .arg("2000-01-01 14:00")
        .arg("test")
        .arg("+tag1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[tag1]"));

    let result = get_frames(&test_db);
    assert_eq!(1, result.len());
    assert_eq!(1, result.get(0).expect("err").tags.values().len());
    assert_eq!(
        "tag1",
        result
            .get(0)
            .expect("err")
            .tags
            .values()
            .get(0)
            .expect("err")
    );
    Ok(())
}

#[test]
fn frames_are_saved_to_database() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add")
        .env("DATABASE_URL", &database)
        .arg("--from")
        .arg("2000-01-01 13:00")
        .arg("--to")
        .arg("2000-01-01 14:00")
        .arg("test");

    cmd.assert().success();

    let result = get_frames(&test_db);
    assert_eq!(1, result.len());

    assert_eq!(
        "2000-01-01 13:00",
        result
            .get(0)
            .unwrap()
            .start
            .format("%Y-%m-%d %H:%M")
            .to_string()
    );

    assert_eq!(
        "2000-01-01 14:00",
        result
            .get(0)
            .unwrap()
            .end
            .expect("err")
            .format("%Y-%m-%d %H:%M")
            .to_string()
    );

    assert_eq!("test", result.get(0).unwrap().project);
    Ok(())
}
