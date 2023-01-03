use assert_cmd::prelude::*;

use chrono::Local;
use predicates::prelude::*;
use std::process::Command;

use crate::{commands::get_frames, TestDb};

use super::add_frame;

#[test]
fn stop_no_project_started() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("aze")?;

    cmd.arg("stop")
        .env("DATABASE_URL", "file::memory:?cache=shared");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No project started"));

    Ok(())
}

#[test]
fn stop_is_stopping_project() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = TestDb::new();
    let database = &test_db.db_path;

    add_frame(&test_db, &"test", &Local::now().naive_local(), None, None)?;

    let mut cmd = Command::cargo_bin("aze")?;

    cmd.env("DATABASE_URL", &database).arg("stop");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Stopping project test"));

    let result = get_frames(&test_db);
    assert_eq!(1, result.len());
    assert!(result.get(0).unwrap().end.is_some());

    Ok(())
}
