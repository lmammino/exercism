const BASE_RATE: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let base_production_rate = speed as u32 * BASE_RATE;
    let success_rate = match speed {
        x if (0..=4).contains(&x) => 1.0,
        x if (5..=8).contains(&x) => 0.9,
        x if (9..=10).contains(&x) => 0.77,
        _ => unreachable!(),
    };

    base_production_rate as f64 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
