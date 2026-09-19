use crate::test_state::prelude::*;

pub static SYMBOLS: LazyLock<SETTINGS_SYMBOLS> = LazyLock::new(|| SETTINGS_SYMBOLS {
    symbols: None,
    coins: Some(vec!["USDT".to_string()]),
    coins_black_list: vec!["-".to_string()],
    ..Default::default()
});
