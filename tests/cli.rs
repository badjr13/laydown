// Tests need to be run in a single thread as multiples tests can access
// the same data file simultaneously. Use: "cargo test -- --test-threads=1"

use assert_cmd::Command;
use chrono::Local;
use std::error::Error;
use std::fs;

use laydown::data_file::get_laydown_data_directory;

type TestResult = Result<(), Box<dyn Error>>;

fn cleanup() -> TestResult {
    let data_file = dirs::data_dir()
        .expect("Failed to find laydown data directory")
        .join("laydown")
        .join("laydown.ron");
    fs::remove_file(data_file)?;

    Ok(())
}

fn run(args: &[&str]) -> TestResult {
    Command::cargo_bin("laydown")?.args(args).assert().success();

    Ok(())
}

fn run_and_assert_stdout(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin("laydown")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn empty_standup() -> TestResult {
    cleanup()?;

    run_and_assert_stdout(&[], "tests/expected/empty_standup.txt")?;

    Ok(())
}

#[test]
fn add_item_to_did() -> TestResult {
    cleanup()?;

    run_and_assert_stdout(&["did", "test did item"], "tests/expected/did_item.txt")?;

    Ok(())
}

#[test]
fn add_items_to_did() -> TestResult {
    cleanup()?;

    run_and_assert_stdout(
        &["did", "test did item 1", "test did item 2"],
        "tests/expected/did_items.txt",
    )?;

    Ok(())
}

#[test]
fn add_item_to_doing() -> TestResult {
    cleanup()?;

    run_and_assert_stdout(
        &["doing", "test doing item"],
        "tests/expected/doing_item.txt",
    )?;

    Ok(())
}

#[test]
fn add_items_to_doing() -> TestResult {
    cleanup()?;

    run_and_assert_stdout(
        &["doing", "test doing item 1", "test doing item 2"],
        "tests/expected/doing_items.txt",
    )?;

    Ok(())
}

#[test]
fn add_item_to_blocker() -> TestResult {
    cleanup()?;

    run_and_assert_stdout(
        &["blocker", "test blocker item"],
        "tests/expected/blocker_item.txt",
    )?;

    Ok(())
}

#[test]
fn add_items_to_blocker() -> TestResult {
    cleanup()?;

    run_and_assert_stdout(
        &["blocker", "test blocker item 1", "test blocker item 2"],
        "tests/expected/blocker_items.txt",
    )?;

    Ok(())
}

#[test]
fn add_item_to_sidebar() -> TestResult {
    cleanup()?;

    run_and_assert_stdout(
        &["sidebar", "test sidebar item"],
        "tests/expected/sidebar_item.txt",
    )?;

    Ok(())
}

#[test]
fn add_items_to_sidebar() -> TestResult {
    cleanup()?;

    run_and_assert_stdout(
        &["sidebar", "test sidebar item 1", "test sidebar item 2"],
        "tests/expected/sidebar_items.txt",
    )?;

    Ok(())
}

#[test]
fn clear() -> TestResult {
    cleanup()?;

    run_and_assert_stdout(
        &["did", "test did item 1", "test did item 2"],
        "tests/expected/did_items.txt",
    )?;

    run(&["--clear"])?;

    run_and_assert_stdout(&[], "tests/expected/empty_standup.txt")?;

    Ok(())
}

// #[test]
// fn edit() -> TestResult {
//
// }

#[test]
fn undo() -> TestResult {
    cleanup()?;

    run_and_assert_stdout(&["did", "test did item"], "tests/expected/did_item.txt")?;

    run_and_assert_stdout(
        &["doing", "test doing item"],
        "tests/expected/did_doing_item.txt",
    )?;

    run(&["--undo"])?;

    run_and_assert_stdout(&[], "tests/expected/did_item.txt")?;

    Ok(())
}

#[test]
fn archive() -> TestResult {
    cleanup()?;

    run_and_assert_stdout(
        &["did", "test did item 1", "test did item 2"],
        "tests/expected/did_items.txt",
    )?;

    run(&["--archive"])?;

    let laydown_data_directory = get_laydown_data_directory();
    let archive_directory = laydown_data_directory.join("archive");
    let date = Local::now().format("%Y-%m-%d").to_string();
    let file_name = format!("{}.txt", date);
    let full_path = archive_directory.join(file_name);

    assert_eq!(
        fs::read_to_string("tests/expected/did_items.txt")?,
        fs::read_to_string(full_path)?,
    );

    Ok(())
}

#[test]
fn test_data_dir() -> TestResult {
    let mut data_directory = dirs::data_dir()
        .expect("Failed to find laydown data directory")
        .join("laydown")
        .into_os_string()
        .into_string()
        .unwrap();

    data_directory.push_str("\n");

    Command::cargo_bin("laydown")?
        .args(["--data-dir"])
        .assert()
        .success()
        .stdout(data_directory);

    Ok(())
}
