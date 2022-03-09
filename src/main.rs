use core::panic;
use std::env;

mod data_file;
mod standup;

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
            CLEAR => data_file::clear_data_from_ron_file(),
            // When directly editing the file, user error could leave the
            // file in a state that cannot be deserialized. All data dependent
            // commands depend on "laydown.ron" being deserializeable and will
            // error out if not. Separating this command allows a user to edit the
            // file that is already in a bad state and fix any issues to recover it.
            EDIT => data_file::manually_edit_ron_file(),
            HELP => print_help_information(),
            _ => run_data_dependent(arguments),
        }
    } else {
        run_data_dependent(arguments)
    }
}

fn run_data_dependent(arguments: Vec<String>) {
    let standup_data = data_file::read_from_ron_file();

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

fn print_invalid_command() {
    println!("The command you entered is not valid. Try \"laydown help\" for a list of commands.")
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
