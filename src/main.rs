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
const STANDUP: &str = "standup";
const HELP: &str = "help";

fn main() {
    let arguments: Vec<String> = env::args().collect();
    // name of executable = arguments[0] by default
    let command = arguments[1].as_str();

    let standup_data = read_from_ron_file();

    match command {
        DID | DOING | BLOCKER | SIDEBAR => standup_data.add_item(command, arguments[2].as_str()),
        CLEAR => standup_data.clear_data(),
        STANDUP => standup_data.display_data(),
        HELP => println!("Help Me Please!"),
        _ => panic!(
            "\"{}\" is not a valid command. Try \"laydown help\" for a list of commands.",
            command
        ),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Standup {
    did: Vec<String>,
    doing: Vec<String>,
    blockers: Vec<String>,
    sidebars: Vec<String>,
}

impl Standup {
    fn new() -> Self {
        Self {
            did: Vec::new(),
            doing: Vec::new(),
            blockers: Vec::new(),
            sidebars: Vec::new(),
        }
    }

    fn add_item(mut self, command: &str, item: &str) {
        match command {
            DID => self.did.push(String::from(item)),
            DOING => self.doing.push(String::from(item)),
            BLOCKER => self.blockers.push(String::from(item)),
            SIDEBAR => self.sidebars.push(String::from(item)),
            _ => println!("Not a valid command."),
        };
        write_to_ron_file(self)
    }

    fn clear_data(self) {
        let file = get_path_to_ron_file();
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file)
            .unwrap();
    }

    fn display_data(self) {
        let standup: Standup = read_from_ron_file();

        if !standup.did.is_empty() {
            println!("DID:");
            for item in standup.did {
                println!("- {}\n", item);
            }
        }

        if !standup.doing.is_empty() {
            println!("DOING:");
            for item in standup.doing {
                println!("- {}\n", item);
            }
        }

        if !standup.blockers.is_empty() {
            println!("BLOCKERS:");
            for item in standup.blockers {
                println!("- {}\n", item);
            }
        }

        if !standup.sidebars.is_empty() {
            println!("SIDEBARS:");
            for item in standup.sidebars {
                println!("- {}\n", item);
            }
        }
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
