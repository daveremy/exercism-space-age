#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    const SECS: f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::SECS
    }
}

#[derive(Planet)]
#[secs-earth-relative = 0.2408467]
pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const SECS: f64 = 0.2408467 * Earth::SECS;
}

impl Planet for Venus {
    const SECS: f64 = 0.61519726 * Earth::SECS;
}

impl Planet for Earth {
    const SECS: f64 = 31557600.0;
}

impl Planet for Mars {
    const SECS: f64 = 1.8808158 * Earth::SECS;
}

impl Planet for Jupiter {
    const SECS: f64 = 11.862615 * Earth::SECS;
}

impl Planet for Saturn {
    const SECS: f64 = 29.447498 * Earth::SECS;
}

impl Planet for Uranus {
    const SECS: f64 = 84.016846 * Earth::SECS;
}

impl Planet for Neptune {
    const SECS: f64 = 164.79132 * Earth::SECS;
}
