use super::fstab_entry::FstabEntry;
use super::parser::{Parser, LoadResult};

#[derive(Clone, Debug)]
pub enum FstabLine {
    EmptyLine,
    Entry(FstabEntry),
    Comment(String)
}

#[derive(Clone, Debug)]
pub struct Fstab {
    pub path: String,
    pub entries: Vec<FstabLine>,
    pub file_exists: bool,
    pub file_corrupted: bool
}

impl Fstab {
    pub fn new(path: &str) -> Self {
        let load_result = Parser::load_from_file(&path);

        match load_result {
            Ok(LoadResult::Loaded(entries)) => Fstab {
                path: path.to_string(),
                entries,
                file_exists: true,
                file_corrupted: false
            },
            Ok(LoadResult::MissingFile) => Fstab {
                path: path.to_string(),
                entries: vec![],
                file_exists: false,
                file_corrupted: false
            },
            _ => Fstab {
                path: path.to_string(),
                entries: vec![],
                file_exists: true,
                file_corrupted: true
            }
        }
    }
}
