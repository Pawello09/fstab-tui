use super::fstab_entry::{FSSpec, FSFile, FSVFSType, FSMntOp, FSFreq, FSPassNo};
use super::fstab_entry::FstabEntry;
use super::fstab::FstabLine;

pub enum LoadResult {
    Loaded(Vec<FstabLine>),
    MissingFile
}

#[derive(Clone, Debug)]
pub struct Parser;

impl Parser {
    pub fn load_from_file(path: &str) -> Result<LoadResult, std::io::Error> {
        match std::fs::read_to_string(path) {
            Ok(content) => Ok(LoadResult::Loaded(Self::parse_content(content.as_str())?)),
            Err(_) => Ok(LoadResult::MissingFile)
        }
    }

    pub fn parse_content(content: &str) -> Result<Vec<FstabLine>, std::io::Error> {
        content.lines().map(Self::parse_line).collect()
    }

    fn parse_line(line: &str) -> Result<FstabLine, std::io::Error> {
        let line_trimmed = line.trim();

        if line_trimmed.is_empty() {
            return Ok(FstabLine::EmptyLine);
        }

        if line.starts_with("#") {
            return Ok(FstabLine::Comment(String::from(&line[1..])));
        }

        let splitted: Vec<&str> = line.split_whitespace().collect();

        if splitted.len() != 6 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData, "invalid amount of fstab fileds"
            ))
        }

        let fs_spec = Self::parse_fs_spec(splitted[0]);
        let fs_file = Self::parse_fs_file(splitted[1]);
        let fs_vfs = Self::parse_fs_vfs_type(splitted[2]);
        let fs_mntops = Self::parse_fs_mntops(splitted[3]);
        let fs_freq = Self::parse_fs_freq(splitted[4]);
        let fs_passno = Self::parse_fs_passno(splitted[5]);

        Ok(
            FstabLine::Entry(FstabEntry {
                fs_spec,
                fs_file,
                fs_vfs,
                fs_mntops,
                fs_freq,
                fs_passno
            })
        )
    }

    fn parse_fs_spec(s: &str) -> FSSpec {
        if let Some(rest) = s.strip_prefix("UUID=") {
            return FSSpec::UUID(rest.to_string());
        }

        if let Some(rest) = s.strip_prefix("LABEL=") {
            return FSSpec::Label(rest.to_string());
        }

        if let Some(rest) = s.strip_prefix("PARTUUID=") {
            return FSSpec::PartUUID(rest.to_string());
        }

        if let Some(rest) = s.strip_prefix("PARTLABEL=") {
            return FSSpec::PartLabel(rest.to_string());
        }

        FSSpec::Custom(s.to_string())
    }

    fn parse_fs_file(s: &str) -> FSFile {
        if s.to_lowercase() == "swap" {
            return FSFile::Swap;
        }

        if s.to_lowercase() == "none" {
            return FSFile::None;
        }

        FSFile::Normal(s.to_string())
    }

    fn parse_fs_vfs_type(s: &str) -> FSVFSType {
        match s {
            "ext4" => FSVFSType::Ext4,
            "xfs" => FSVFSType::Xfs,
            "btrfs" => FSVFSType::Btrfs,
            "f2fs" => FSVFSType::F2fs,
            other => FSVFSType::Custom(other.to_string())
        }
    }

    fn parse_fs_mntops(s: &str) -> Vec<FSMntOp> {
        s.split(',').filter(|op| !op.is_empty()).map(|op|
            match op {
                "defaults" => FSMntOp::Defaults,
                "rw" => FSMntOp::RW,
                "ro" => FSMntOp::RO,
                "exec" => FSMntOp::Exec,
                "noexec" => FSMntOp::NoExec,
                "sync" => FSMntOp::Sync,
                "async" => FSMntOp::Async,
                "auto" => FSMntOp::Auto,
                "noauto" => FSMntOp::NoAuto,
                other => FSMntOp::Custom(other.to_string())
            }
        ).collect()
    }

    fn parse_fs_freq(s: &str) -> FSFreq {
        match s {
            "0" => FSFreq::NoDump,
            "1" => FSFreq::Dump,
            _ => FSFreq::DumpWithLowPriority
        }
    }

    fn parse_fs_passno(s: &str) -> FSPassNo {
        match s {
            "0" => FSPassNo::NoCheck,
            "1" => FSPassNo::CheckRoot,
            _ => FSPassNo::CheckOther
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_empty_line_parse() -> Result<(), std::io::Error> {
        let result = Parser::parse_line("")?;
        assert_eq!(result, FstabLine::EmptyLine);
        Ok(())
    }

    #[test]
    fn it_comment_line_parse() -> Result<(), std::io::Error> {
        let result = Parser::parse_line("# this is a comment")?;
        assert_eq!(result, FstabLine::Comment(" this is a comment".to_string()));
        Ok(())
    }

    #[test]
    fn it_normal_line_parse() -> Result<(), std::io::Error> {
        let result = Parser::parse_line("UUID=4d028ac1-d413-4d3a-94b4-251db287744f / ext4 rw,auto 1 1")?;
        assert_eq!(result, FstabLine::Entry(
            FstabEntry {
                fs_spec: FSSpec::UUID("4d028ac1-d413-4d3a-94b4-251db287744f".to_string()),
                fs_file: FSFile::Normal("/".to_string()),
                fs_vfs: FSVFSType::Ext4,
                fs_mntops: vec![FSMntOp::RW, FSMntOp::Auto],
                fs_freq: FSFreq::Dump,
                fs_passno: FSPassNo::CheckRoot
            }
        ));
        Ok(())
    }
}
