use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::data_file;
use crate::print_standup_data;
use crate::{BL, BLOCKER, DI, DID, DO, DOING, SB, SIDEBAR};

#[derive(Debug, Deserialize, Serialize)]
pub struct Standup {
    pub did: Vec<String>,
    pub doing: Vec<String>,
    pub blockers: Vec<String>,
    pub sidebars: Vec<String>,
    pub history: Vec<String>,
}

impl Standup {
    pub fn new() -> Self {
        Self {
            did: Vec::new(),
            doing: Vec::new(),
            blockers: Vec::new(),
            sidebars: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn add_item(mut self, file: &Path, command: &str, item: &str) {
        match command {
            DID | DI => {
                self.did.push(String::from(item));
                self.history.push(DID.to_string());
            }
            DOING | DO => {
                self.doing.push(String::from(item));
                self.history.push(DOING.to_string());
            }
            BLOCKER | BL => {
                self.blockers.push(String::from(item));
                self.history.push(BLOCKER.to_string());
            }
            SIDEBAR | SB => {
                self.sidebars.push(String::from(item));
                self.history.push(SIDEBAR.to_string());
            }
            _ => println!("Not a valid command."),
        };

        data_file::write_to_file(file, self);
        print_standup_data(file)
    }

    pub fn undo(mut self, file: &Path) {
        match self.history.pop().expect("No history available").as_str() {
            DID | DI => self.did.pop(),
            DOING | DO => self.doing.pop(),
            BLOCKER | BL => self.blockers.pop(),
            SIDEBAR | SB => self.sidebars.pop(),
            _ => Some("Invalid History".to_string()),
        };

        data_file::write_to_file(file, self);
        print_standup_data(file)
    }
}

// Clippy recommended I do this but I don't understand why yet.
// See: https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/new_without_default.rs
impl Default for Standup {
    fn default() -> Self {
        Self::new()
    }
}
