pub(super) fn scaled_points(value: u8, maximum: u8) -> u8 {
    ((u16::from(value) * u16::from(maximum)) / 100) as u8
}

pub(super) fn data_volume_points(bytes: u64) -> u8 {
    match bytes {
        0 => 0,
        1..=1_024 => 2,
        1_025..=1_048_576 => 10,
        1_048_577..=1_073_741_824 => 20,
        _ => 30,
    }
}

pub(super) fn financial_amount_points(minor_units: u64) -> u8 {
    match minor_units {
        0 => 0,
        1..=10_000 => 2,
        10_001..=1_000_000 => 15,
        1_000_001..=100_000_000 => 25,
        _ => 35,
    }
}
