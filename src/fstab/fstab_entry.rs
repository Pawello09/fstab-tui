use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FSSpec {
    Label(String),
    UUID(String),
    PartUUID(String),
    PartLabel(String),
    Custom(String)
}

impl fmt::Display for FSSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FSSpec::Label(label) => write!(f, "LABEL={}", label),
            FSSpec::UUID(uuid) => write!(f, "UUID={}", uuid),
            FSSpec::PartLabel(part_label) => write!(f, "PARTLABEL={}", part_label),
            FSSpec::PartUUID(part_uuid) => write!(f, "PARTUUID={}", part_uuid),
            FSSpec::Custom(other) => write!(f, "{}", other)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FSFile {
    Normal(String),
    None,
    Swap
}

impl fmt::Display for FSFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FSFile::Swap => write!(f, "swap"),
            FSFile::None => write!(f, "none"),
            FSFile::Normal(path) => write!(f, "{}", path)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FSVFSType {
    Ext4,
    Xfs,
    Btrfs,
    F2fs,
    Custom(String)
}

impl fmt::Display for FSVFSType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FSVFSType::Ext4 => write!(f, "ext4"),
            FSVFSType::Xfs => write!(f, "xfs"),
            FSVFSType::Btrfs => write!(f, "btrfs"),
            FSVFSType::F2fs => write!(f, "f2fs"),
            FSVFSType::Custom(other) => write!(f, "{}", other)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FSMntOp {
    Defaults,
    RW,
    RO,
    Auto,
    NoAuto,
    Exec,
    NoExec,
    Sync,
    Async,
    Custom(String)
}

impl fmt::Display for FSMntOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FSMntOp::Defaults => write!(f, "defaults"),
            FSMntOp::RW => write!(f, "rw"),
            FSMntOp::RO => write!(f, "ro"),
            FSMntOp::Auto => write!(f, "auto"),
            FSMntOp::NoAuto => write!(f, "noauto"),
            FSMntOp::Exec => write!(f, "exec"),
            FSMntOp::NoExec => write!(f, "noexec"),
            FSMntOp::Sync => write!(f, "sync"),
            FSMntOp::Async => write!(f, "async"),
            FSMntOp::Custom(other) => write!(f, "{}", other)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FSFreq {
    NoDump,
    Dump,
    DumpWithLowPriority
}

impl fmt::Display for FSFreq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FSFreq::NoDump => write!(f, "0"),
            FSFreq::Dump => write!(f, "1"),
            FSFreq::DumpWithLowPriority => write!(f, "2")
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FSPassNo {
    NoCheck,
    CheckRoot,
    CheckOther
}

impl fmt::Display for FSPassNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FSPassNo::NoCheck => write!(f, "0"),
            FSPassNo::CheckRoot => write!(f, "1"),
            FSPassNo::CheckOther => write!(f, "2")
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FstabEntry {
    pub fs_spec: FSSpec,
    pub fs_file: FSFile,
    pub fs_vfs: FSVFSType,
    pub fs_mntops: Vec<FSMntOp>,
    pub fs_freq: FSFreq,
    pub fs_passno: FSPassNo
}

impl fmt::Display for FstabEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {} {} {}",
            self.fs_spec.to_string(),
            self.fs_file.to_string(),
            self.fs_vfs.to_string(),
            self.fs_mntops.iter().map(|mntops| mntops.to_string()).collect::<Vec<String>>().join(","),
            self.fs_freq.to_string(),
            self.fs_passno.to_string()
        )
    }
}
