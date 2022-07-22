use std::fs;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use chrono::Local;

use crate::standup::Standup;

use crate::Env;

pub fn get_laydown_config_directory() -> PathBuf {
    dirs::config_dir()
        .expect("Failed to find laydown config directory")
        .join("laydown")
}

pub fn get_path_to_file(env: Env) -> PathBuf {
    let laydown_config_directory = get_laydown_config_directory();
    fs::create_dir(&laydown_config_directory).ok();

    let ron_data_file: PathBuf = match env {
        Env::Prod => laydown_config_directory.join("laydown.ron"),
        Env::Test => laydown_config_directory.join("test_laydown.ron"),
    };

    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&ron_data_file)
        .expect("Failed to find laydown.ron file");

    ron_data_file
}

pub fn read_from_file(file: &Path) -> Standup {
    let content = fs::read_to_string(file).expect("Failed to read content from data file.");

    if content.len() == 0 {
        let new_standup = Standup::new();
        write_to_file(file, &new_standup);
        new_standup
    } else {
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
}

pub fn write_to_file(file: &Path, data: &Standup) {
    let warning = "// Do not rename or delete arrays. Only update elements.\n".to_string();

    let standup_data = ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default())
        .expect("Failed to serialize laydown.ron Struct to string");

    let content = warning + &standup_data;

    fs::write(file, content).expect("Failed to write to laydown.ron");
}

pub fn manually_edit_file(file: &Path, editor: String) {
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

pub fn archive(file: &Path) {
    let laydown_config_directory = get_laydown_config_directory();
    let archive_directory = laydown_config_directory.join("archive");
    fs::create_dir(&archive_directory).ok();

    let date = Local::now().format("%Y-%m-%d").to_string();

    let file_name = format!("{}.txt", date);
    let full_path = archive_directory.join(file_name);

    let standup: Standup = read_from_file(file);

    fs::write(full_path, standup.to_string()).expect("Failed to write archive file.");

    clear_data_from_file(file);
}
