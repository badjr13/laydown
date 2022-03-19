use std::fs;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use crate::standup::Standup;

use crate::Env;

pub fn get_path_to_file(env: Env) -> PathBuf {
    match env {
        Env::Prod => {
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
        Env::Test => {
            let laydown_test_directory = dirs::config_dir()
                .expect("Failed to find laydown config directory")
                .join("laydown_test");

            fs::create_dir(&laydown_test_directory).ok();

            let test_file_path: PathBuf = laydown_test_directory.join("laydown_test.ron");

            OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(&test_file_path)
                .expect("Failed to find laydown.ron file");

            test_file_path
        }
    }
}

pub fn read_from_file(file: &Path) -> Standup {
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

pub fn write_to_file(file: &Path, data: Standup) {
    let warning = "// Do not rename or delete arrays. Only update elements.\n".to_string();

    let standup_data = ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default())
        .expect("Failed to serialize laydown.ron Struct to string");

    let content = warning + &standup_data;

    fs::write(file, content).expect("Failed to write to laydown.ron");
}

pub fn manually_edit_file(file: &Path, editor: &str) {
    match Command::new(editor).arg(file).status() {
        Ok(edit_file) => edit_file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Could not find editor provided."),
            other_error => panic!("{:?}", other_error),
        },
    };
}

pub fn clear_data_from_file(file: &Path) {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file)
        .expect("Failed to erase existing data from laydown.ron");
}
