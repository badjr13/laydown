// Tests need to be run in a single thread as multiples tests can access
// the same data file simultaneously. Use: "cargo test -- --test-threads=1"

use std::fs;
use std::path::PathBuf;

use laydown::data_file;
use laydown::parse_arguments;
use laydown::Env;

const FAKE: &str = "fake/binary";

fn test_file() -> PathBuf {
    data_file::get_path_to_file(Env::Test)
}

fn delete_test_data_file() {
    let laydown_test_file = dirs::config_dir()
        .expect("Failed to find laydown_test config directory")
        .join("laydown/test_laydown.ron");
    fs::remove_file(laydown_test_file).unwrap();
}

#[test]
#[should_panic(expected = "Something went horribly wrong.")]
fn test_app_failed_to_run() {
    let arguments = vec![];
    parse_arguments(arguments, Env::Test);
}

#[test]
fn test_no_arguments_passed() {
    let arguments = vec![FAKE.to_string()];
    parse_arguments(arguments, Env::Test);
    delete_test_data_file();
}

#[test]
fn test_add_item() {
    let test_data = "test item added to did";
    let arguments = vec![FAKE.to_string(), "did".to_string(), test_data.to_string()];
    parse_arguments(arguments, Env::Test);

    let standup = data_file::read_from_file(&test_file());

    assert_eq!(standup.did.len(), 1);
    assert_eq!(standup.did[0], test_data.to_string());

    delete_test_data_file();
}

#[test]
fn test_clear() {
    let test_data = "test item added to doing";

    let arguments = vec![FAKE.to_string(), "do".to_string(), test_data.to_string()];
    parse_arguments(arguments, Env::Test);

    let standup = data_file::read_from_file(&test_file());
    assert_eq!(standup.doing.len(), 1);
    assert_eq!(standup.doing[0], test_data.to_string());

    let arguments_for_clear = vec![FAKE.to_string(), "clear".to_string()];
    parse_arguments(arguments_for_clear, Env::Test);

    let standup = data_file::read_from_file(&test_file());
    assert_eq!(standup.doing.len(), 0);

    delete_test_data_file();
}
