use assert_cmd::prelude::*;
use chrono::{Local, NaiveDateTime};
use predicates::prelude::*;
use std::process::Command;

use crate::TestDb;

use super::add_frame;

#[test]
fn get_only_correct_tag_entries() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &test_db,
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        Some(&NaiveDateTime::from_timestamp(end, 0)),
        Some(vec!["test1".to_string()]),
    )?;
    add_frame(
        &test_db,
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        Some(&NaiveDateTime::from_timestamp(end + 3600, 0)),
        Some(vec!["test2".to_string()]),
    )?;
    let mut cmd = Command::cargo_bin("aze")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-T")
        .arg("test1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2").not());

    Ok(())
}

#[test]
fn get_only_correct_tags_entries() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &test_db,
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        Some(&NaiveDateTime::from_timestamp(end, 0)),
        Some(vec!["test1".to_string()]),
    )?;
    add_frame(
        &test_db,
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        Some(&NaiveDateTime::from_timestamp(end + 3600, 0)),
        Some(vec!["test2".to_string()]),
    )?;

    let mut cmd = Command::cargo_bin("aze")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-T")
        .arg("test1")
        .arg("-T")
        .arg("test2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2"));

    Ok(())
}

#[test]
fn ignore_single_tag() -> Result<(), Box<dyn std::error::Error>> {
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
        Some(vec!["test2".to_string()]),
    )?;

    let mut cmd = Command::cargo_bin("aze")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("--ignore-tag")
        .arg("test2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2").not());

    Ok(())
}

#[test]
fn ignore_multiple_tags() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &test_db,
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        Some(&NaiveDateTime::from_timestamp(end, 0)),
        Some(vec!["test1".to_string()]),
    )?;
    add_frame(
        &test_db,
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        Some(&NaiveDateTime::from_timestamp(end + 3600, 0)),
        Some(vec!["test2".to_string()]),
    )?;

    let mut cmd = Command::cargo_bin("aze")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("--ignore-tag")
        .arg("test1")
        .arg("--ignore-tag")
        .arg("test2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1").not())
        .stdout(predicate::str::contains("test2").not());

    Ok(())
}

#[test]
fn ignore_and_select_tags() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let mut cmd = Command::cargo_bin("aze")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-T")
        .arg("test1")
        .arg("--ignore-tag")
        .arg("test1");

    cmd.assert().failure().stderr(predicate::str::contains(
        "given tags can't be ignored at the same time",
    ));

    Ok(())
}

#[test]
fn get_entries_from_multiple_tags() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let start = Local::now().naive_local().timestamp() - 7200;
    let end = start + 1800;

    add_frame(
        &test_db,
        &"test1",
        &NaiveDateTime::from_timestamp(start, 0),
        Some(&NaiveDateTime::from_timestamp(end, 0)),
        Some(vec!["test1".to_string(), "test3".to_string()]),
    )?;
    add_frame(
        &test_db,
        &"test2",
        &NaiveDateTime::from_timestamp(start + 3600, 0),
        Some(&NaiveDateTime::from_timestamp(end + 3600, 0)),
        Some(vec!["test2".to_string(), "test3".to_string()]),
    )?;
    let mut cmd = Command::cargo_bin("aze")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-T")
        .arg("test3");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2"));

    Ok(())
}
