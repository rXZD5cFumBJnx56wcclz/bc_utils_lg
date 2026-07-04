#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Signal {
    pub signal: f64,
    pub probability: f64,
}

impl Signal {
    pub fn new(signal: f64, probability: f64) -> Self {
        Self {
            signal,
            probability,
        }
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self::new(0., 1.)
    }
}