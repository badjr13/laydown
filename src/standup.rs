use serde::{Deserialize, Serialize};

use crate::data_file;
use crate::{BLOCKER, DID, DOING, SIDEBAR};

#[derive(Debug, Deserialize, Serialize)]
pub struct Standup {
    pub did: Vec<String>,
    pub doing: Vec<String>,
    pub blockers: Vec<String>,
    pub sidebars: Vec<String>,
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

        data_file::write_to_ron_file(self);

        crate::display_data();
    }
}
