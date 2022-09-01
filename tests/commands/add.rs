use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn add_without_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let file_path = dir.path().join("frames_test.db");

    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add").env("DATABASE_URL", file_path);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("arguments were not provided"));

    dir.close()?;
    Ok(())
}

#[test]
fn add_frame_with_correct_argument() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let file_path = dir.path().join("frames_test.db");

    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("add")
        .env("DATABASE_URL", file_path)
        .arg("--from")
        .arg("2000-01-01 12:00")
        .arg("--to")
        .arg("2000-01-01 13:00")
        .arg("test");
    cmd.assert().success().stdout(predicate::str::contains(
        "starting project test from 2000-01-01 12:00 to 2000-01-01 13:00",
    ));

    dir.close()?;
    Ok(())
}
