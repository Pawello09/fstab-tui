use super::fstab_entry::FstabEntry;

#[derive(Clone, Debug)]
pub struct Parser;

impl Parser {
    pub fn load_from_file(path: &str) -> Vec<FstabEntry> {
        return vec![];
    }
}
