use crate::test_state::prelude::*;

// fix
// slippage
pub static TRADE: LazyLock<SETTINGS_TRADE> = LazyLock::new(|| SETTINGS_TRADE {
    capital: 1000.,
    work_in_real_time: true,
    klines_qty: 10_000,
    leverage: 10.,
    mode_trade: "isolated".to_string(),
    mode_hedge: true,
    time_in_force: "GTC".to_string(),
    commission_market: 0.001,
    commission_limit: 0.001,
    signal_hold: 0.,
    signal_long: 1.,
    signal_short: -1.,
    ..Default::default()
});
