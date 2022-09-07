use assert_cmd::prelude::*;
use chrono::{Local, NaiveDateTime};
use predicates::prelude::*;
use std::process::Command;

use crate::TestDb;

use super::add_frame;

#[test]
fn get_only_correct_project_entries() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-p")
        .arg("test1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2").not());

    Ok(())
}

#[test]
fn get_only_correct_projects_entries() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-p")
        .arg("test1")
        .arg("-p")
        .arg("test2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2"));

    Ok(())
}

#[test]
fn ignore_single_project() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("--ignore-project")
        .arg("test2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1"))
        .stdout(predicate::str::contains("test2").not());

    Ok(())
}

#[test]
fn ignore_multiple_projects() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("--ignore-project")
        .arg("test1")
        .arg("--ignore-project")
        .arg("test2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test1").not())
        .stdout(predicate::str::contains("test2").not());

    Ok(())
}

#[test]
fn ignore_and_select_projects() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;
    let mut cmd = Command::cargo_bin("mycroft")?;
    cmd.env("DATABASE_URL", &database)
        .arg("log")
        .arg("-p")
        .arg("test1")
        .arg("--ignore-project")
        .arg("test1");

    cmd.assert().failure().stderr(predicate::str::contains(
        "given projects can't be ignored at the same time",
    ));

    Ok(())
}
