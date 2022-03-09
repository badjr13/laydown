use serde::{Deserialize, Serialize};

use crate::data_file;
use crate::{BLOCKER, DID, DOING, SIDEBAR};

#[derive(Debug, Deserialize, Serialize)]
pub struct Standup {
    did: Vec<String>,
    doing: Vec<String>,
    blockers: Vec<String>,
    sidebars: Vec<String>,
}

impl Standup {
    pub fn new() -> Self {
        Self {
            did: Vec::new(),
            doing: Vec::new(),
            blockers: Vec::new(),
            sidebars: Vec::new(),
        }
    }

    pub fn add_item(mut self, command: &str, item: &str) {
        match command {
            DID => self.did.push(String::from(item)),
            DOING => self.doing.push(String::from(item)),
            BLOCKER => self.blockers.push(String::from(item)),
            SIDEBAR => self.sidebars.push(String::from(item)),
            _ => println!("Not a valid command."),
        };
        data_file::write_to_ron_file(self)
    }

    pub fn display_data(self) {
        let standup: Standup = data_file::read_from_ron_file();

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
