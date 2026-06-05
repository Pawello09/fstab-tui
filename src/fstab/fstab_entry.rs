#[derive(Clone, Debug)]
pub enum FSSpec {
    Label(String),
    UUID(String),
    PartUUID(String),
    PartLabel(String)
}

#[derive(Clone, Debug)]
pub enum FSFile {
    Normal(String),
    Swap
}

#[derive(Clone, Debug)]
pub enum FSVFSType {
    Ext4,
    Xfs,
    Btrfs,
    F2fs,
    Custom(String)
}

#[derive(Clone, Debug)]
pub enum FSMntOp {
    Custom(String)
}

#[derive(Clone, Debug)]
pub enum FSFreq {
    NoDump,
    Dump,
    DumpWithLowPriority
}

#[derive(Clone, Debug)]
pub enum FSPassNo {
    NoCheck,
    CheckRoot,
    CheckOther
}

#[derive(Clone, Debug)]
pub struct FstabEntry {
    fs_spec_type: FSSpec,
    fs_file_type: FSFile,
    fs_vfs_type: FSVFSType,
    fs_mntops: Vec<FSMntOp>,
    fs_freq: FSFreq,
    fs_passno: FSPassNo
}
