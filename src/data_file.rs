use std::fs;
use std::fs::OpenOptions;
use std::io::{stdin, ErrorKind};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use chrono::Local;

use crate::standup::Standup;

use crate::Env;

pub fn get_laydown_data_directory() -> PathBuf {
    dirs::data_dir()
        .expect("Failed to find laydown data directory")
        .join("laydown")
}

pub fn get_path_to_file(env: Env) -> PathBuf {
    let laydown_data_directory = get_laydown_data_directory();
    fs::create_dir(&laydown_data_directory).ok();

    let ron_data_file: PathBuf = match env {
        Env::Prod => laydown_data_directory.join("laydown.ron"),
        Env::Test => laydown_data_directory.join("test_laydown.ron"),
    };

    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&ron_data_file)
        .expect("Failed to find laydown.ron file");

    ron_data_file
}

fn fix_missing_history(content: &mut String) -> Standup {
    let pos = content
        .rfind(',')
        .expect("File seems broken, please consider fixing it with 'laydown edit'")
        + 1;
    content.insert_str(pos, "history: [],\n");

    match ron::from_str(content) {
        Ok(_deserialized_content) => _deserialized_content,
        Err(e) => panic!("Failed to fix laydown.ron: {}", e),
    }
}

fn deserialize_ron_file(content: &mut String) -> Standup {
    let deserialized_content: Standup = match ron::from_str(content) {
        Ok(_deserialized_content) => _deserialized_content,
        Err(error) => match error.code {
            ron::error::ErrorCode::ExpectedStruct => Standup::new(),
            ron::error::ErrorCode::Message(s) => {
                let str_s = s.as_str();
                match str_s {
                    "missing field `history`" => fix_missing_history(content),
                    _ => panic!("Failed to deserialize content from laydown.ron: {}", s),
                }
            }
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

pub fn get_standup(file: &Path) -> Standup {
    let mut content = fs::read_to_string(file).expect("Failed to read content from data file.");

    if content.is_empty() {
        let new_standup = Standup::new();
        write_to_file(file, &new_standup);
        new_standup
    } else {
        deserialize_ron_file(&mut content)
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
    let laydown_data_directory = get_laydown_data_directory();
    let archive_directory = laydown_data_directory.join("archive");
    fs::create_dir(&archive_directory).ok();

    let date = Local::now().format("%Y-%m-%d").to_string();

    let file_name = format!("{}.txt", date);
    let full_path = archive_directory.join(file_name);

    if full_path.exists() {
        overwrite_existing_archive(file, &full_path);
    } else {
        let standup: Standup = get_standup(file);
        fs::write(full_path, standup.to_string()).expect("Failed to write archive file.");
        clear_data_from_file(file);
    }
}

#[allow(clippy::needless_return)]
fn overwrite_existing_archive(file: &Path, full_path: &PathBuf) {
    println!("An archive already exists for today. Would you like to overwrite today's existing archive file? (y/n)");

    let mut user_input = String::new();

    stdin()
        .read_line(&mut user_input)
        .expect("Type 'y' for yes or 'n' for no.");

    if user_input.trim_end() == "y" {
        let standup: Standup = get_standup(file);
        fs::write(full_path, standup.to_string()).expect("Failed to write archive file.");
        clear_data_from_file(file);
    } else if user_input.trim_end() == "n" {
        return;
    } else {
        println!("Type 'y' for yes or 'n' for no.");
    }
}
