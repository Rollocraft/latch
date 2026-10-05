#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactorKind {
    Baseline,
    Environment,
    Sensitivity,
    Reversibility,
    DataVolume,
    Trust,
    HistoricalAnomaly,
    DestinationTrust,
    FinancialAmount,
    BlastRadius,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Contribution {
    pub(super) factor: FactorKind,
    pub(super) points: u8,
    pub(super) unknown: bool,
}

impl Contribution {
    pub fn factor(&self) -> FactorKind {
        self.factor
    }
    pub fn points(&self) -> u8 {
        self.points
    }
    pub fn is_unknown(&self) -> bool {
        self.unknown
    }
}
