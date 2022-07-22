use std::env;

pub mod data_file;
mod standup;

const CLEAR: &str = "clear";
const EDIT: &str = "edit";
const UNDO: &str = "undo";
const ARCHIVE: &str = "archive";
const CONFIG_DIR: &str = "config-dir";

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
    let standup = data_file::read_from_file(&file);

    match arguments.len() {
        // This should never happen. The name of the binary
        // is always the first element of env::args
        0 => panic!("Something went horribly wrong."),
        1 => print!("{}", standup),
        2 => match arguments[1].as_str() {
            CLEAR => data_file::clear_data_from_file(&file),
            EDIT => {
                let default_editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
                data_file::manually_edit_file(&file, default_editor)
            }
            UNDO => standup.undo(&file),
            ARCHIVE => data_file::archive(&file),
            CONFIG_DIR => println!("{}", data_file::get_laydown_config_directory().display()),
            HELP | DASH_HELP => print_help_information(),
            _ => print_invalid_command(),
        },
        3.. => {
            let command = arguments[1].as_str();
            let (_, user_input) = arguments.split_at(2);
            match command {
                DID | DI | DOING | DO | BLOCKER | BL | SIDEBAR | SB => {
                    standup.add_item(&file, command, user_input.to_vec())
                }
                EDIT => data_file::manually_edit_file(&file, arguments[2].to_string()),
                _ => print_invalid_command(),
            }
        }
        _ => print_invalid_command(),
    }
}

fn print_help_information() {
    println!("\nRunning \"laydown\" without passing any arguments will display your Standup\n");
    println!("Usage: laydown <command> \"<item>\" \"<item>\"\n");
    println!("Available commands:");
    println!("di, did      <item>  Add item to DID section of your Standup");
    println!("do, doing    <item>  Add item to DOING section of your Standup");
    println!("bl, blocker  <item>  Add item to BLOCKERS section of your Standup");
    println!("sb, sidebar  <item>  Add item to SIDEBARS section of your Standup\n");
    println!("                     TIP: Multiple space separated items can be added.\n");
    println!("clear                Remove all items from your Standup");
    println!("edit <editor>        Directly access data displayed in your Standup.");
    println!("                     This can be used to edit or delete existing entries.");
    println!("                     Will use VI by default if no editor is provided.");
    println!("undo                 Remove last item added to your Standup.\n");
    println!("archive              Archive today's Standup. Found in laydown config directory.");
    println!("                     Note: Archiving will automatically clear out existing Standup.");
    println!("config-dir           Print location of laydown config directory.\n");
    println!("help, --help         Display this message\n");
}

fn print_invalid_command() {
    println!("The command you entered is not valid. Try \"laydown help\" for a list of commands.")
}
