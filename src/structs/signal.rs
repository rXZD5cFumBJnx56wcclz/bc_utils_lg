use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Signal {
    pub signal: f64,
    pub probability: f64,
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "signal: {}, prob: {}", self.signal, self.probability)
    }
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
