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
    did: Option<Vec<String>>,
    doing: Option<Vec<String>>,
    blocked: Option<Vec<String>>,
    sidebar: Option<Vec<String>>,
    clear: bool,
    edit: bool,
    undo: bool,
    archive: bool,
    data_dir: bool,
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
                .num_args(1..)
                .display_order(1)
        )
        .arg(
            Arg::new("doing")
                .help("Add item(s) to DOING section of your Standup")
                .short('d')
                .long("doing")
                .value_name("ITEM")
                .num_args(1..)
                .display_order(2)
        )
        .arg(
            Arg::new("blocked")
                .help("Add item(s) to BLOCKED section of your Standup")
                .short('b')
                .long("blocked")
                .value_name("ITEM")
                .num_args(1..)
                .display_order(3)
        )
        .arg(
            Arg::new("sidebar")
                .help("Add item(s) to SIDEBAR section of your Standup")
                .short('s')
                .long("sidebar")
                .value_name("ITEM")
                .num_args(1..)
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
                .long("data-dir")
                .action(clap::ArgAction::SetTrue)
                .display_order(9)
        )
        .get_matches();

    let did: Vec<String> = matches
        .get_many::<String>("did")
        .unwrap_or_default()
        .map(|x| x.to_string())
        .collect();

    let doing: Vec<String> = matches
        .get_many::<String>("doing")
        .unwrap_or_default()
        .map(|x| x.to_string())
        .collect();

    let blocked: Vec<String> = matches
        .get_many::<String>("blocked")
        .unwrap_or_default()
        .map(|x| x.to_string())
        .collect();

    let sidebar: Vec<String> = matches
        .get_many::<String>("sidebar")
        .unwrap_or_default()
        .map(|x| x.to_string())
        .collect();

    let clear: bool = matches.get_flag("clear");
    let edit: bool = matches.get_flag("edit");
    let undo: bool = matches.get_flag("undo");
    let archive: bool = matches.get_flag("archive");
    let data_dir: bool = matches.get_flag("data_dir");

    Ok(Config {
        did: if did.is_empty() { None } else { Some(did) },
        doing: if doing.is_empty() { None } else { Some(doing) },
        blocked: if blocked.is_empty() { None } else { Some(blocked) },
        sidebar: if sidebar.is_empty() { None } else { Some(sidebar) },
        clear,
        edit,
        undo,
        archive,
        data_dir,
    })
}

pub fn run(config: Config) -> LaydownResult<()> {
    let file = data_file::get_path_to_file(Env::Prod);

    if let Some(items) = config.did {
        data_file::get_standup(&file).add_item(&file, DID, items);
    }
    if let Some(items) = config.doing {
        data_file::get_standup(&file).add_item(&file, DOING, items);
    }
    if let Some(items) = config.blocked {
        data_file::get_standup(&file).add_item(&file, BLOCKER, items);
    }
    if let Some(items) = config.sidebar {
        data_file::get_standup(&file).add_item(&file, SIDEBAR, items);
    }
    if config.clear {
        data_file::clear_data_from_file(&file)
    }
    if config.edit {
        let default_editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        data_file::manually_edit_file(&file, default_editor);
    }
    if config.undo {
        data_file::get_standup(&file).undo(&file);
    }
    if config.archive {
        data_file::archive(&file);
    }
    if config.data_dir {
        println!("{}", data_file::get_laydown_config_directory().display());
    }

    Ok(())
}

fn print_invalid_command() {
    println!("The command you entered is not valid. Try \"laydown help\" for a list of commands.")
}
