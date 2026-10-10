use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_TRADE {
    pub capital: f64,
    pub work_in_real_time: bool,
    pub klines_qty: usize,
    pub leverage: f64,
    pub mode_trade: String,
    pub mode_hedge: bool,
    pub slippage_tolerance_type: String,
    pub slippage_tolerance: (f64, f64),
    pub time_in_force: String,
    pub commission_market: f64,
    pub commission_limit: f64,
    pub signal_hold: f64,
    pub signal_long: f64,
    pub signal_short: f64,
}

impl Default for SETTINGS_TRADE {
    fn default() -> Self {
        Self {
            capital: 1000.,
            work_in_real_time: false,
            klines_qty: Default::default(),
            leverage: 1.0,
            mode_trade: "isolated".to_string(),
            mode_hedge: true,
            slippage_tolerance_type: Default::default(),
            slippage_tolerance: Default::default(),
            time_in_force: "GTC".to_string(),
            commission_market: 0.001,
            commission_limit: 0.001,
            signal_hold: 0.,
            signal_long: 1.,
            signal_short: -1.,
        }
    }
}