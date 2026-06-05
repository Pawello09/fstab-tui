use super::fstab_entry::FstabEntry;
use super::parser::Parser;

#[derive(Clone, Debug)]
pub struct Fstab {
    pub path: String,
    pub entries: Vec<FstabEntry>
}

impl Fstab {
    pub fn new(path: &str) -> Self {
        let entries = Parser::load_from_file(&path);

        Fstab {
            path: path.to_string(),
            entries
        }
    }
}
