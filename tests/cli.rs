use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const LAYDOWN: &str = "laydown";

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(LAYDOWN)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
