use std::fs;
use std::fs::OpenOptions;
use std::io::{stdin, ErrorKind};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use chrono::Local;

use crate::standup::Standup;

pub fn get_laydown_data_directory() -> PathBuf {
    let data_directory = dirs::data_dir()
        .expect("Failed to find laydown data directory")
        .join("laydown");

    fs::create_dir(&data_directory).ok();

    data_directory
}

pub fn get_path_to_laydown_data_file() -> PathBuf {
    let data_directory = get_laydown_data_directory();

    let data_file = data_directory.join("laydown.ron");

    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&data_file)
        .expect("Failed to find laydown data file");

    data_file
}

fn fix_missing_history(content: &mut String) -> Standup {
    let pos = content
        .rfind(',')
        .expect("Data file is out of sync. Fix using '--edit' or '--clear'.")
        + 1;
    content.insert_str(pos, "history: [],\n");

    match ron::from_str(content) {
        Ok(_deserialized_content) => _deserialized_content,
        Err(e) => panic!("Failed to fix laydown data file: {}", e),
    }
}

fn deserialize_data_file(content: &mut String) -> Standup {
    let deserialized_content: Standup = match ron::from_str(content) {
        Ok(_deserialized_content) => _deserialized_content,
        Err(error) => match error.code {
            ron::error::ErrorCode::ExpectedStruct => Standup::new(),
            ron::error::ErrorCode::Message(serialization_error) => {
                match serialization_error.as_str() {
                    "missing field `history`" => fix_missing_history(content),
                    _ => panic!(
                        "Failed to deserialize content from laydown data file: {}",
                        serialization_error
                    ),
                }
            }
            other_error => {
                panic!(
                    "Failed to deserialize content from laydown data file: {}",
                    other_error
                );
            }
        },
    };
    deserialized_content
}

pub fn get_standup(data_file: &Path) -> Standup {
    let mut content =
        fs::read_to_string(data_file).expect("Failed to read content from data file.");

    if content.is_empty() {
        let new_standup = Standup::new();
        write_to_file(data_file, &new_standup);
        new_standup
    } else {
        deserialize_data_file(&mut content)
    }
}

pub fn write_to_file(data_file: &Path, data: &Standup) {
    let warning = "// Do not rename or delete arrays. Only update elements.\n".to_string();

    let standup_data = ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default())
        .expect("Failed to serialize laydown data file Struct to String");

    let content = warning + &standup_data;

    fs::write(data_file, content).expect("Failed to write to laydown data file");
}

pub fn manually_edit_file(data_file: &Path, editor: String) {
    match Command::new(editor).arg(data_file).status() {
        Ok(edit_file) => edit_file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!(
                "No editor passed. $EDITOR variable not set.'vi' is default but not installed."
            ),
            other_error => panic!("{:?}", other_error),
        },
    };
}

pub fn clear_data_from_file(data_file: &Path) {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(data_file)
        .expect("Failed to erase existing data from laydown data file");
}

pub fn archive(data_file: &Path) {
    let data_directory = get_laydown_data_directory();
    let archive_directory = data_directory.join("archive");
    fs::create_dir(&archive_directory).ok();

    let date = Local::now().format("%Y-%m-%d").to_string();

    let file_name = format!("{}.txt", date);
    let full_path = archive_directory.join(file_name);

    if full_path.exists() {
        overwrite_existing_archive(data_file, &full_path);
    } else {
        let standup: Standup = get_standup(data_file);
        fs::write(full_path, standup.to_string()).expect("Failed to write archive file.");
        clear_data_from_file(data_file);
    }
}

#[allow(clippy::needless_return)]
fn overwrite_existing_archive(data_file: &Path, full_path: &PathBuf) {
    println!("An archive already exists for today. Would you like to overwrite today's existing archive file? (y/n)");

    let mut user_input = String::new();

    stdin()
        .read_line(&mut user_input)
        .expect("Type 'y' for yes or 'n' for no.");

    if user_input.trim_end() == "y" {
        let standup: Standup = get_standup(data_file);
        fs::write(full_path, standup.to_string()).expect("Failed to write archive file.");
        clear_data_from_file(data_file);
    } else if user_input.trim_end() == "n" {
        return;
    } else {
        println!("Type 'y' for yes or 'n' for no.");
    }
}
