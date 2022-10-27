use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;

use crate::data_file;
use crate::{BLOCKER, DID, DOING, SIDEBAR};

#[derive(Deserialize, Serialize)]
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

    pub fn add_item(mut self, file: &Path, command: &str, items: Vec<String>) {
        match command {
            DID => {
                for item in items {
                    self.did.push(item);
                    self.history.push(DID.to_string());
                }
            }
            DOING => {
                for item in items {
                    self.doing.push(item);
                    self.history.push(DOING.to_string());
                }
            }
            BLOCKER => {
                for item in items {
                    self.blockers.push(item);
                    self.history.push(BLOCKER.to_string());
                }
            }
            SIDEBAR => {
                for item in items {
                    self.sidebars.push(item);
                    self.history.push(SIDEBAR.to_string());
                }
            }
            _ => println!("Not a valid command."),
        };

        data_file::write_to_file(file, &self);
        print!("{}", self);
    }

    pub fn undo(mut self, file: &Path) {
        match self.history.pop().expect("No history available").as_str() {
            DID => self.did.pop(),
            DOING => self.doing.pop(),
            BLOCKER => self.blockers.pop(),
            SIDEBAR => self.sidebars.pop(),
            _ => Some("Invalid History".to_string()),
        };

        data_file::write_to_file(file, &self);
        print!("{}", self);
    }
}

impl fmt::Display for Standup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DID:")?;
        if !&self.did.is_empty() {
            for item in &self.did {
                writeln!(f, "- {}", item)?;
            }
            writeln!(f)?;
        } else {
            writeln!(f, "**empty**\n")?;
        }

        writeln!(f, "DOING:")?;
        if !&self.doing.is_empty() {
            for item in &self.doing {
                writeln!(f, "- {}", item)?;
            }
            writeln!(f)?;
        } else {
            writeln!(f, "**empty**\n")?;
        }

        writeln!(f, "BLOCKERS:")?;
        if !&self.blockers.is_empty() {
            for item in &self.blockers {
                writeln!(f, "- {}", item)?;
            }
            writeln!(f)?;
        } else {
            writeln!(f, "**empty**\n")?;
        }

        writeln!(f, "SIDEBARS:")?;
        if !&self.sidebars.is_empty() {
            for item in &self.sidebars {
                writeln!(f, "- {}", item)?;
            }
        } else {
            writeln!(f, "**empty**")?;
        }

        Ok(())
    }
}

// Clippy recommended I do this but I don't understand why yet.
// See: https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/new_without_default.rs
impl Default for Standup {
    fn default() -> Self {
        Self::new()
    }
}
