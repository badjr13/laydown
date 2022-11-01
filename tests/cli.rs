// Tests need to be run in a single thread as multiples tests can access
// the same data file simultaneously. Use: "cargo test -- --test-threads=1"

use assert_cmd::Command;
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

fn cleanup() -> TestResult {
    let data_file = dirs::data_dir()
        .expect("Failed to find laydown data directory")
        .join("laydown")
        .join("laydown.ron");

    fs::remove_file(data_file)?;

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
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
    run(&[], "tests/expected/empty_standup.txt")
}

#[test]
fn add_item_to_did() -> TestResult {
    cleanup()?;
    run(&["did", "test did item"], "tests/expected/did_item.txt")
}

#[test]
fn add_items_to_did() -> TestResult {
    cleanup()?;
    run(
        &["did", "test did item 1", "test did item 2"],
        "tests/expected/did_items.txt",
    )
}

#[test]
fn add_item_to_doing() -> TestResult {
    cleanup()?;
    run(
        &["doing", "test doing item"],
        "tests/expected/doing_item.txt",
    )
}

#[test]
fn add_items_to_doing() -> TestResult {
    cleanup()?;
    run(
        &["doing", "test doing item 1", "test doing item 2"],
        "tests/expected/doing_items.txt",
    )
}

#[test]
fn add_item_to_blocker() -> TestResult {
    cleanup()?;
    run(
        &["blocker", "test blocker item"],
        "tests/expected/blocker_item.txt",
    )
}

#[test]
fn add_items_to_blocker() -> TestResult {
    cleanup()?;
    run(
        &["blocker", "test blocker item 1", "test blocker item 2"],
        "tests/expected/blocker_items.txt",
    )
}

#[test]
fn add_item_to_sidebar() -> TestResult {
    cleanup()?;
    run(
        &["sidebar", "test sidebar item"],
        "tests/expected/sidebar_item.txt",
    )
}

#[test]
fn add_items_to_sidebar() -> TestResult {
    cleanup()?;
    run(
        &["sidebar", "test sidebar item 1", "test sidebar item 2"],
        "tests/expected/sidebar_items.txt",
    )
}

// #[test]
// fn clear() -> TestResult {
//     cleanup()?;
//
//     run(
//         &["did", "test did item 1", "test did item 2"],
//         "tests/expected/did_items.txt",
//     );
//
//     run(&["--clear"], "tests/expected/empty_standup.txt");
//
//     Ok(())
// }
