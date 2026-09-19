use crate::test_state::prelude::*;

pub static EXCH: LazyLock<SETTINGS_EXCH> = LazyLock::new(|| SETTINGS_EXCH {
    url: "https://api-testnet.bybit.com".to_string(),
    wws_host: "stream-testnet.bybit.com".to_string(),
    wws_url: "wss://stream-testnet.bybit.com".to_string(),
    exchange: "bybit".to_string(),
    account_type: "UNIFIED".to_string(),
    timeframe_sec: Duration::from_mins(1),
    category: "linear".to_string(),
    timeout_req_ms: Duration::from_secs(3),
    timeout_cycle_ms: Duration::from_secs(5),
    ping_ms: Duration::from_secs(10),
    timeout_socket_ms: Duration::from_secs(1),
    ..Default::default()
});
