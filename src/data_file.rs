use std::fs;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

use crate::standup::Standup;

fn get_path_to_ron_file() -> PathBuf {
    let laydown_config_directory = dirs::config_dir()
        .expect("Failed to find laydown config directory")
        .join("laydown");

    fs::create_dir(&laydown_config_directory).ok();

    let ron_file_path: PathBuf = laydown_config_directory.join("laydown.ron");

    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&ron_file_path)
        .expect("Failed to find laydown.ron file");

    ron_file_path
}

#[test]
fn test_get_path_to_ron_file() {
    let result: PathBuf = get_path_to_ron_file();

    let expected: PathBuf = dirs::config_dir()
        .unwrap()
        .join("laydown")
        .join("laydown.ron");

    assert_eq!(result.to_str().unwrap(), expected.to_str().unwrap());
}

pub fn read_from_ron_file() -> Standup {
    let file = get_path_to_ron_file();

    let content = fs::read_to_string(file).expect("Failed to read content from laydown.ron");

    let deserialized_content: Standup = match ron::from_str(&content) {
        Ok(_deserialized_content) => _deserialized_content,
        Err(error) => match error.code {
            ron::error::ErrorCode::ExpectedStruct => Standup::new(),
            other_error => {
                panic!(
                    "Failed to deserialize content from laydown.ron: {}",
                    other_error
                );
            }
        },
    };

    deserialized_content
}

pub fn write_to_ron_file(data: Standup) {
    let file = get_path_to_ron_file();

    let warning = "// Do not rename or delete arrays. Only update elements.\n".to_string();

    let standup_data = ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default())
        .expect("Failed to serialize laydown.ron Struct to string");

    let content = warning + &standup_data;

    fs::write(file, content).expect("Failed to write to laydown.ron");
}

pub fn manually_edit_ron_file(editor: &str) {
    let file = get_path_to_ron_file();

    match Command::new(editor).arg(file).status() {
        Ok(edit_file) => edit_file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Could not find editor provided."),
            other_error => panic!("{:?}", other_error),
        },
    };
}

pub fn clear_data_from_ron_file() {
    let file = get_path_to_ron_file();

    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file)
        .expect("Failed to erase existing data from laydown.ron");
}
