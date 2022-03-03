use core::panic;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;

const DID: &str = "did";
const DOING: &str = "doing";
const BLOCKER: &str = "blocker";
const SIDEBAR: &str = "sidebar";

const CLEAR: &str = "clear";

fn main() {
    let arguments: Vec<String> = env::args().collect();
    // name of executable = arguments[0]
    let command = arguments[1].as_str();
    // let item = arguments[2].as_str();
    let item = "wow";

    let standup_data = read_from_ron_file();

    match command {
        DID | DOING | BLOCKER | SIDEBAR => standup_data.add_item(command, item),
        CLEAR => standup_data.clear(),
        _ => panic!(
            "\"{}\" is not a valid command. Try \"laydown help\" for a list of commands.",
            command
        ),
    }
}

fn get_path_to_ron_file() -> PathBuf {
    let laydown_config_directory = dirs::config_dir().unwrap().join("laydown");
    fs::create_dir(&laydown_config_directory).ok();

    let ron_file_path: PathBuf = laydown_config_directory.join("laydown.ron");

    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&ron_file_path)
        .unwrap();

    ron_file_path
}

#[derive(Debug, Deserialize, Serialize)]
struct Standup {
    did: Vec<String>,
    doing: Vec<String>,
    blockers: Vec<String>,
    sidebar: Vec<String>,
}

impl Standup {
    fn new() -> Self {
        Self {
            did: Vec::new(),
            doing: Vec::new(),
            blockers: Vec::new(),
            sidebar: Vec::new(),
        }
    }

    fn add_item(mut self, command: &str, item: &str) {
        match command {
            DID => self.did.push(String::from(item)),
            DOING => self.doing.push(String::from(item)),
            BLOCKER => self.blockers.push(String::from(item)),
            SIDEBAR => self.sidebar.push(String::from(item)),
            _ => println!("Not a valid command."),
        };
        write_to_ron_file(self)
    }

    fn clear(self) {
        let file = get_path_to_ron_file();
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file)
            .unwrap();
    }
}

fn read_from_ron_file() -> Standup {
    let file = get_path_to_ron_file();
    let content = fs::read_to_string(file).unwrap();
    let deserialized_content: Standup = match ron::from_str(&content) {
        Ok(good_stuff) => good_stuff,
        Err(error) => match error.code {
            ron::error::ErrorCode::ExpectedStruct => Standup::new(),
            other_error => {
                panic!(
                    "There was an issue when trying to read the ron file: {}",
                    other_error
                );
            }
        },
    };
    deserialized_content
}

fn write_to_ron_file(data: Standup) {
    let file = get_path_to_ron_file();
    let content = ron::to_string(&data).unwrap();
    fs::write(file, content).unwrap();
}
