const SECS_IN_YEAR: f64 = 60.0 * 60.0 * 24.0 * 365.25;

#[derive(Debug)]
pub struct Duration(pub u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn time_ratio_to_hearth() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / Self::time_ratio_to_hearth() / SECS_IN_YEAR
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn time_ratio_to_hearth() -> f64 {
        0.2408467
    }
}

impl Planet for Venus {
    fn time_ratio_to_hearth() -> f64 {
        0.61519726
    }
}

impl Planet for Earth {
    fn time_ratio_to_hearth() -> f64 {
        1.0
    }
}
impl Planet for Mars {
    fn time_ratio_to_hearth() -> f64 {
        1.8808158
    }
}

impl Planet for Jupiter {
    fn time_ratio_to_hearth() -> f64 {
        11.862615
    }
}

impl Planet for Saturn {
    fn time_ratio_to_hearth() -> f64 {
        29.447498
    }
}

impl Planet for Uranus {
    fn time_ratio_to_hearth() -> f64 {
        84.016846
    }
}

impl Planet for Neptune {
    fn time_ratio_to_hearth() -> f64 {
        164.79132
    }
}
