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
    edit: bool,
    undo: bool,
    archive: bool,
    config_dir: bool,
}

pub fn get_args() -> LaydownResult<Config> {
    let matches = Command::new("laydown")
        .version("2.0.0")
        .author("Bobby Dorrance")
        .about("laydown is a simple CLI application to help you prepare for your next Daily Standup. No longer shall your name be called on only for you to stare into the abyss while you struggle to remember what you did yesterday.")
        .arg(
            Arg::new("did")
                .help("Add item(s) to DID section of your Standup")
                .short('x')
                .long("did")
                .value_name("ITEM")
                .display_order(1)
        )
        .arg(
            Arg::new("doing")
                .help("Add item(s) to DOING section of your Standup")
                .short('d')
                .long("doing")
                .value_name("ITEM")
                .display_order(2)
        )
        .arg(
            Arg::new("blocked")
                .help("Add item(s) to BLOCKED section of your Standup")
                .short('b')
                .long("blocked")
                .value_name("ITEM")
                .display_order(3)
        )
        .arg(
            Arg::new("sidebar")
                .help("Add item(s) to SIDEBAR section of your Standup")
                .short('s')
                .long("sidebar")
                .value_name("ITEM")
                .display_order(4)
        )
        .arg(
            Arg::new("clear")
                .help("Remove all items from your Standup")
                .long("clear")
                .action(clap::ArgAction::SetTrue)
                .display_order(5)
        )
        .arg(
            Arg::new("edit")
                .help("Directly access/edit data in your Standup")
                .long("edit")
                .action(clap::ArgAction::SetTrue)
                .display_order(6)
        )
        .arg(
            Arg::new("undo")
                .help("Remove last item added to your standup")
                .long("undo")
                .action(clap::ArgAction::SetTrue)
                .display_order(7)
        )
        .arg(
            Arg::new("archive")
                .help("Archive today's Standup.")
                .long("archive")
                .action(clap::ArgAction::SetTrue)
                .display_order(8)
        )
        .arg(
            Arg::new("data_dir")
                .help("Print location of the laydown data directory")
                .long("dat_dir")
                .action(clap::ArgAction::SetTrue)
                .display_order(9)
        )
        .get_matches();

    let did = Some("did test".to_string());
    let doing = Some("do test".to_string());
    let blocker = Some("blocker test".to_string());
    let sidebar = Some("sidebar test".to_string());
    let clear = false;
    let edit = false;
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

fn print_invalid_command() {
    println!("The command you entered is not valid. Try \"laydown help\" for a list of commands.")
}
