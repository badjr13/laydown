use clap::{Arg, Command};

pub mod data_file;
mod standup;

const DID: &str = "did";
const DOING: &str = "doing";
const BLOCKER: &str = "blocker";
const SIDEBAR: &str = "sidebar";

type LaydownResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Config {
    did: Option<Vec<String>>,
    doing: Option<Vec<String>>,
    blocker: Option<Vec<String>>,
    sidebar: Option<Vec<String>>,
    clear: bool,
    edit: Option<String>,
    undo: bool,
    archive: bool,
    data_dir: bool,
}

pub fn get_args() -> LaydownResult<Config> {
    let matches = Command::new("laydown")
        .version("2.6.2")
        .author("Bobby Dorrance")
        .about("Keep track of your next Daily Stand Up")
        .disable_help_subcommand(true)
        .override_usage("laydown [COMMAND] \"<item>\" \"<item>\" \"<item>\"")
        .subcommand(
            Command::new(DID)
                .about("Add items to the DID section of your standup")
                .arg(Arg::new("items").num_args(1..)),
        )
        .subcommand(
            Command::new(DOING)
                .about("Add items to the DOING section of your standup")
                .arg(Arg::new("items").num_args(1..)),
        )
        .subcommand(
            Command::new(BLOCKER)
                .about("Add items to the BLOCKER section of your standup")
                .arg(Arg::new("items").num_args(1..)),
        )
        .subcommand(
            Command::new(SIDEBAR)
                .about("Add items to the SIDEBAR section of your standup")
                .arg(Arg::new("items").num_args(1..)),
        )
        .arg(
            Arg::new("clear")
                .help("Remove all items from your Standup")
                .long("clear")
                .action(clap::ArgAction::SetTrue)
                .display_order(5),
        )
        .arg(
            Arg::new("edit")
                .help("Directly access/edit data in your Standup")
                .long("edit")
                .value_name("EDITOR")
                .num_args(0..=1)
                .default_missing_value("vi")
                .display_order(6),
        )
        .arg(
            Arg::new("undo")
                .help("Remove last item added to your Standup")
                .long("undo")
                .action(clap::ArgAction::SetTrue)
                .display_order(7),
        )
        .arg(
            Arg::new("archive")
                .help("Archive today's Standup")
                .long("archive")
                .action(clap::ArgAction::SetTrue)
                .display_order(8),
        )
        .arg(
            Arg::new("data_dir")
                .help("Print location of the laydown data directory")
                .long("data-dir")
                .action(clap::ArgAction::SetTrue)
                .display_order(9),
        )
        .get_matches();

    let did = match matches.subcommand() {
        Some((DID, x)) => {
            let items: Vec<String> = x
                .get_many::<String>("items")
                .unwrap_or_default()
                .map(|x| x.to_string())
                .collect();
            Some(items)
        }
        _ => None,
    };

    let doing = match matches.subcommand() {
        Some((DOING, x)) => {
            let items: Vec<String> = x
                .get_many::<String>("items")
                .unwrap_or_default()
                .map(|x| x.to_string())
                .collect();
            Some(items)
        }
        _ => None,
    };

    let blocker = match matches.subcommand() {
        Some((BLOCKER, x)) => {
            let items: Vec<String> = x
                .get_many::<String>("items")
                .unwrap_or_default()
                .map(|x| x.to_string())
                .collect();
            Some(items)
        }
        _ => None,
    };

    let sidebar = match matches.subcommand() {
        Some((SIDEBAR, x)) => {
            let items: Vec<String> = x
                .get_many::<String>("items")
                .unwrap_or_default()
                .map(|x| x.to_string())
                .collect();
            Some(items)
        }
        _ => None,
    };

    let clear: bool = matches.get_flag("clear");

    let edit = matches
        .get_one::<String>("edit")
        .map(|editor| editor.to_owned());

    let undo: bool = matches.get_flag("undo");
    let archive: bool = matches.get_flag("archive");
    let data_dir: bool = matches.get_flag("data_dir");

    Ok(Config {
        did,
        doing,
        blocker,
        sidebar,
        clear,
        edit,
        undo,
        archive,
        data_dir,
    })
}

pub fn run(config: Config) -> LaydownResult<()> {
    let file = data_file::get_path_to_laydown_data_file();

    let mut show_standup_if_no_args_present = true;

    if let Some(items) = config.did {
        data_file::get_standup(&file).add_item(&file, DID, items);
        show_standup_if_no_args_present = false;
    }
    if let Some(items) = config.doing {
        data_file::get_standup(&file).add_item(&file, DOING, items);
        show_standup_if_no_args_present = false;
    }
    if let Some(items) = config.blocker {
        data_file::get_standup(&file).add_item(&file, BLOCKER, items);
        show_standup_if_no_args_present = false;
    }
    if let Some(items) = config.sidebar {
        data_file::get_standup(&file).add_item(&file, SIDEBAR, items);
        show_standup_if_no_args_present = false;
    }
    if config.clear {
        data_file::clear_data_from_file(&file);
        show_standup_if_no_args_present = false;
    }
    if let Some(editor) = config.edit {
        match std::env::var("EDITOR") {
            Ok(user_default_editor) => data_file::manually_edit_file(&file, user_default_editor),
            Err(_) => data_file::manually_edit_file(&file, editor),
        }
        show_standup_if_no_args_present = false;
    }
    if config.undo {
        data_file::get_standup(&file).undo(&file);
        show_standup_if_no_args_present = false;
    }
    if config.archive {
        data_file::archive(&file);
        show_standup_if_no_args_present = false;
    }
    if config.data_dir {
        println!("{}", data_file::get_laydown_data_directory().display());
        show_standup_if_no_args_present = false;
    }
    if show_standup_if_no_args_present {
        let test = data_file::get_standup(&file);
        print!("{}", test);
    }

    Ok(())
}
