use std::env;
use std::path::Path;

pub mod data_file;
mod standup;

const CLEAR: &str = "clear";
const EDIT: &str = "edit";
const HELP: &str = "help";
const DASH_HELP: &str = "--help";

const DID: &str = "did";
const DI: &str = "di";

const DOING: &str = "doing";
const DO: &str = "do";

const BLOCKER: &str = "blocker";
const BL: &str = "bl";

const SIDEBAR: &str = "sidebar";
const SB: &str = "sb";

pub enum Env {
    Prod,
    Test,
}

pub fn parse_arguments(arguments: Vec<String>, env: Env) {
    let file = data_file::get_path_to_file(env);

    match arguments.len() {
        // This should never happen. The name of the binary
        // is always the first element of env::args
        0 => panic!("Something went horribly wrong."),
        1 => print_standup_data(&file),
        2 => {
            let default_editor =
                env::var("EDITOR").expect("No $EDITOR environment variable found.");
            match arguments[1].as_str() {
                CLEAR => data_file::clear_data_from_file(&file),
                EDIT => data_file::manually_edit_file(&file, default_editor),
                HELP | DASH_HELP => print_help_information(),
                _ => print_invalid_command(),
            }
        }
        3 => {
            let command = arguments[1].as_str();
            let user_input = arguments[2].as_str();
            let standup = data_file::read_from_file(&file);
            match command {
                DID | DI | DOING | DO | BLOCKER | BL | SIDEBAR | SB => {
                    // TODO! Fix Using Moved Value error below
                    standup.add_item(&file, command, user_input)
                }
                EDIT => data_file::manually_edit_file(&file, user_input.to_string()),
                _ => print_invalid_command(),
            }
        }
        _ => print_invalid_command(),
    }
}

fn print_standup_data(file: &Path) {
    let standup = data_file::read_from_file(file);

    println!();

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

fn print_help_information() {
    println!("\nRunning \"laydown\" without passing any arguments will display your Standup\n");
    println!("Usage: laydown <command> \"<item>\"\n");
    println!("Available commands:");
    println!("di, did      <item>  Add item to DID section of your Standup");
    println!("do, doing    <item>  Add item to DOING section of your Standup");
    println!("bl, blocker  <item>  Add item to BLOCKERS section of your Standup");
    println!("sb, sidebar  <item>  Add item to SIDEBARS section of your Standup\n");
    println!("clear                Remove all items from your Standup\n");
    println!("edit <editor>        Directly access data displayed in your Standup.");
    println!("                     This can be used to edit or delete existing entries.");
    println!("                     Will use VI by default if no editor is provided.\n");
    println!("help, --help         Display this message\n");
}

fn print_invalid_command() {
    println!("The command you entered is not valid. Try \"laydown help\" for a list of commands.")
}
