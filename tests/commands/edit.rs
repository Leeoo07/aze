use assert_cmd::prelude::*;

use chrono::Local;
use predicates::prelude::*;
use std::{process::{Command, Stdio}, fs::File};

use crate::TestDb;
use std::io::Read;
use super::add_frame;

#[test]
fn edit_no_project_started() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mycroft")?;

    cmd.arg("status")
        .env("DATABASE_URL", "file::memory:?cache=shared");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No project started"));

    Ok(())
}
