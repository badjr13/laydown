use core::panic;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::process::Command;

const CLEAR: &str = "clear";
const EDIT: &str = "edit";
const HELP: &str = "help";

const DID: &str = "did";
const DOING: &str = "doing";
const BLOCKER: &str = "blocker";
const SIDEBAR: &str = "sidebar";
const STANDUP: &str = "standup";

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() == 2 {
        // These commands are data independent
        match arguments[1].as_str() {
            // When directly editing the file, user error could leave the
            // file in a state that cannot be deserialized. All data dependent
            // commands depend on "laydown.ron" being deserializeable and will
            // error out if not. Separating this command allows a user to edit the
            // file that is already in a bad state and fix any issues to recover it.
            CLEAR => clear_data_from_ron_file(),
            EDIT => manually_edit_ron_file(),
            HELP => print_help_information(),
            _ => (),
        }
    } else {
        run_data_dependent(arguments)
    }
}

fn print_invalid_command() {
    println!("The command you entered is not valid. Try \"laydown help\" for a list of commands.")
}

fn run_data_dependent(arguments: Vec<String>) {
    let standup_data = read_from_ron_file();

    match arguments.len() {
        0 => panic!("Something went wrong."),
        1 => standup_data.display_data(),
        2 => {
            let command = arguments[1].as_str();
            match command {
                STANDUP => standup_data.display_data(),
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

    fn display_data(self) {
        let standup: Standup = read_from_ron_file();

        if !standup.did.is_empty() {
            println!("DID:");
            for item in standup.did {
                println!("- {}", item);
            }
            println!();
        }

        if !standup.doing.is_empty() {
            println!("DOING:");
            for item in standup.doing {
                println!("- {}", item);
            }
            println!();
        }

        if !standup.blockers.is_empty() {
            println!("BLOCKERS:");
            for item in standup.blockers {
                println!("- {}", item);
            }
            println!();
        }

        if !standup.sidebars.is_empty() {
            println!("SIDEBARS:");
            for item in standup.sidebars {
                println!("- {}", item);
            }
            println!();
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
    let content = ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default())
        .expect("Failed to serialize laydown.ron Struct to string");
    fs::write(file, content).expect("Failed to write to laydown.ron");
}

fn manually_edit_ron_file() {
    let file = get_path_to_ron_file();
    Command::new("vi")
        .arg(file)
        .status()
        .expect("Failed to open laydown.ron");
}

fn clear_data_from_ron_file() {
    let file = get_path_to_ron_file();
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file)
        .expect("Failed to erase existing data from laydown.ron");
}

fn print_help_information() {
    println!("\nRunning \"laydown\" without passing any arguments will display your Standup\n");
    println!("Usage: laydown <command> \"<item>\"\n");
    println!("Available commands:");
    println!("did         Add item to DID section of your Standup");
    println!("doing       Add item to DOING section of your Standup");
    println!("blocker     Add item to BLOCKERS section of your Standup");
    println!("sidebar     Add item to SIDEBARS section of your Standup\n");
    println!("clear       Remove all items from your Standup\n");
    println!("edit        Directly access data displayed in your Standup.");
    println!("            This can be used to edit or delete existing entries.");
    println!("            CAUTION: Edits must follow RON formatting. ");
    println!("            See: https://github.com/ron-rs/ron\n");
    println!("help        Display this message\n");
}
