#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    Filesystem,
    Process,
    Network,
    CpuTime,
    Memory,
    ProcessCount,
    Disk,
    Runtime,
    Freeze,
    Terminate,
}

impl Control {
    pub const ALL: [Self; 10] = [
        Self::Filesystem,
        Self::Process,
        Self::Network,
        Self::CpuTime,
        Self::Memory,
        Self::ProcessCount,
        Self::Disk,
        Self::Runtime,
        Self::Freeze,
        Self::Terminate,
    ];

    pub(crate) fn index(self) -> usize {
        match self {
            Self::Filesystem => 0,
            Self::Process => 1,
            Self::Network => 2,
            Self::CpuTime => 3,
            Self::Memory => 4,
            Self::ProcessCount => 5,
            Self::Disk => 6,
            Self::Runtime => 7,
            Self::Freeze => 8,
            Self::Terminate => 9,
        }
    }
}
