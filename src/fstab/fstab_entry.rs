#[derive(Clone, Debug)]
pub enum FSSpec {
    Label(String),
    UUID(String),
    PartUUID(String),
    PartLabel(String),
    Custom(String)
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
    pub fs_spec: FSSpec,
    pub fs_file: FSFile,
    pub fs_vfs: FSVFSType,
    pub fs_mntops: Vec<FSMntOp>,
    pub fs_freq: FSFreq,
    pub fs_passno: FSPassNo
}
