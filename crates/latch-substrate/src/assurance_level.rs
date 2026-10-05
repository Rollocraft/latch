#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssuranceLevel {
    CooperativeMediation,
    OsConfinement,
}

impl AssuranceLevel {
    pub fn satisfies(self, required: Self) -> bool {
        self == required || self == Self::OsConfinement
    }
}
