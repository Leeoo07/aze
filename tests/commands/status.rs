use assert_cmd::prelude::*;

use chrono::Local;
use predicates::prelude::*;
use std::process::Command;

use crate::TestDb;

use super::add_frame;

#[test]
fn status_no_project_started() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("aze")?;

    cmd.arg("status")
        .env("DATABASE_URL", "file::memory:?cache=shared");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No project started"));

    Ok(())
}

#[test]
fn status_project_started() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;

    add_frame(&test_db, &"test", &Local::now().naive_local(), None, None)?;

    let mut cmd = Command::cargo_bin("aze")?;

    cmd.env("DATABASE_URL", &database).arg("status");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Project test started"));

    Ok(())
}
