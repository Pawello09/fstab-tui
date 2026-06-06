use std::fmt;
use super::fstab_entry::FstabEntry;
use super::parser::{Parser, LoadResult};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FstabLine {
    EmptyLine,
    Entry(FstabEntry),
    Comment(String)
}

impl fmt::Display for FstabLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
            match self {
                FstabLine::Comment(comment) => "#".to_string() + comment,
                FstabLine::Entry(entry) => entry.to_string(),
                _ => "".to_string(),
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Fstab {
    pub path: String,
    pub lines: Vec<FstabLine>,
    pub selected_line: Option<usize>,
    pub file_exists: bool,
    pub file_corrupted: bool
}

impl Fstab {
    pub fn new(path: &str) -> Self {
        let load_result = Parser::load_from_file(&path);

        match load_result {
            Ok(LoadResult::Loaded(lines)) => Fstab {
                path: path.to_string(),
                lines,
                selected_line: None,
                file_exists: true,
                file_corrupted: false
            },
            Ok(LoadResult::MissingFile) => Fstab {
                path: path.to_string(),
                lines: vec![],
                selected_line: None,
                file_exists: false,
                file_corrupted: false
            },
            _ => Fstab {
                path: path.to_string(),
                lines: vec![],
                selected_line: None,
                file_exists: true,
                file_corrupted: true
            }
        }
    }
}
