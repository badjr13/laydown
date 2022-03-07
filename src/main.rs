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

fn print_invalid_command() {
    println!("The command you entered is not valid. Try \"laydown help\" for a list of commands.")
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let standup_data = read_from_ron_file();

    match arguments.len() {
        0 => panic!("Something went wrong."),
        // Name of executable is always passed as the first element of env::args()
        // User arguments start at the second element or argument[1]
        1 => standup_data.display_data(),
        2 => {
            let command = arguments[1].as_str();
            match command {
                STANDUP => standup_data.display_data(),
                CLEAR => standup_data.clear_data(),
                HELP => print_help_information(),
                _ => print_invalid_command(),
            }
        }
        3 => {
            let command = arguments[1].as_str();
            let user_input = arguments[2].as_str();
            match command {
                DID | DOING | BLOCKER | SIDEBAR => standup_data.add_item(command, user_input),
                _ => print_invalid_command(),
            }
        }
        _ => print_invalid_command(),
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
            .expect("Failed to erase existing data from laydown.ron");
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

fn read_from_ron_file() -> Standup {
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

fn write_to_ron_file(data: Standup) {
    let file = get_path_to_ron_file();
    let content = ron::to_string(&data).expect("Failed to serialize laydown.ron Struct to string");
    fs::write(file, content).expect("Failed to write to laydown.ron");
}

fn print_help_information() {
    println!("\nUsage: laydown <command> \"<argument>\"\n");
    println!("Available commands:");
    println!("did         Add item to DID section of your Standup");
    println!("doing       Add item to DOING section of your Standup");
    println!("blocker     Add item to BLOCKERS section of your Standup");
    println!("sidebar     Add item to SIDEBARS section of your Standup\n");
    println!("clear       Remove all items from your Standup");
    println!("help        Display this message\n");
    println!("Running 'laydown' without passing any arguments will display your Standup")
}
