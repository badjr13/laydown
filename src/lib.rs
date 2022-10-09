use clap::{Arg, Command};
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

type LaydownResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Config {
    did: Option<String>,
    doing: Option<String>,
    blocker: Option<String>,
    sidebar: Option<String>,
    clear: bool,
    edit: Option<String>,
    undo: bool,
    archive: bool,
    config_dir: bool,
}

pub fn get_args() -> LaydownResult<Config> {
    let matches = Command::new("laydown")
        .version("2.0.0")
        .author("Bobby Dorrance")
        .about("laydown is a simple CLI application to help you prepare for your next Daily Standup. No longer shall your name be called on only for you to stare into the abyss while you struggle to remember what you did yesterday.");

        let did = Some("did test".to_string());
        let doing = Some("do test".to_string());
        let blocker = Some("blocker test".to_string());
        let sidebar = Some("sidebar test".to_string());
        let clear = false;
        let edit = Some("edit test".to_string());
        let undo = false;
        let archive = false;
        let config_dir = false;


    Ok(Config {
        did,
        doing,
        blocker,
        sidebar,
        clear,
        edit,
        undo,
        archive,
        config_dir,
    })
}

pub fn run(config: Config) -> LaydownResult<()> {
    println!("{:?}", config);
    Ok(())
}


pub fn parse_arguments(arguments: Vec<String>, env: Env) {
    let file = data_file::get_path_to_file(env);

    match arguments.len() {
        // This should never happen. The name of the binary
        // is always the first element of env::args
        0 => panic!("Something went horribly wrong."),
        1 => print!("{}", data_file::get_standup(&file)),
        2 => match arguments[1].as_str() {
            CLEAR => data_file::clear_data_from_file(&file),
            EDIT => {
                let default_editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
                data_file::manually_edit_file(&file, default_editor)
            }
            UNDO => data_file::get_standup(&file).undo(&file),
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
                    data_file::get_standup(&file).add_item(&file, command, user_input.to_vec())
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
