use assert_cmd::prelude::*;

use chrono::Local;
use predicates::prelude::*;
use std::process::Command;

use crate::TestDb;

use super::add_frame;

#[test]
fn start_requires_project() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("aze")?;

    cmd.arg("start")
        .env("DATABASE_URL", "file::memory:?cache=shared");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("arguments were not provided"));

    Ok(())
}

#[test]
fn start_with_project() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("aze")?;

    cmd.env("DATABASE_URL", "file::memory:?cache=shared")
        .arg("start")
        .arg("test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("starting project test"));

    Ok(())
}

#[test]
fn start_with_project_but_already_started() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;

    add_frame(
        &test_db,
        &"another",
        &Local::now().naive_local(),
        None,
        None,
    )?;

    let mut cmd = Command::cargo_bin("aze")?;

    cmd.env("DATABASE_URL", &database).arg("start").arg("test");

    cmd.assert().failure().stderr(predicate::str::contains(
        "Project another is already started",
    ));

    Ok(())
}

#[test]
fn start_project_with_tags() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("aze")?;

    cmd.env("DATABASE_URL", "file::memory:?cache=shared")
        .arg("start")
        .arg("test")
        .arg("+tag1")
        .arg("+tag2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("tag1"))
        .stdout(predicate::str::contains("tag2"));

    Ok(())
}
