use crate::test_state::prelude::*;

// fix
pub static ORDER_FILTERS: LazyLock<SETTINGS_ORDER_FILTERS> = LazyLock::new(|| {
    SETTINGS_ORDER_FILTERS::from_iter([
        (
            "count_1".to_string(),
            SETTINGS_ORDER_FILTER {
                key: "count".to_string(),
                kwargs_usize: MAP::from_iter([("max_count".to_string(), 1)]),
                used_orders: vec!["open_order".to_string()],
                ..Default::default()
            },
        ),
        (
            "count_2".to_string(),
            SETTINGS_ORDER_FILTER {
                key: "count".to_string(),
                kwargs_usize: MAP::from_iter([("max_count".to_string(), 1)]),
                used_orders: vec!["avg_order".to_string()],
                use_in_trade: true,
                ..Default::default()
            },
        ),
        (
            "count_3".to_string(),
            SETTINGS_ORDER_FILTER {
                key: "count".to_string(),
                kwargs_usize: MAP::from_iter([("max_count".to_string(), 1)]),
                used_orders: vec!["sl".to_string()],
                use_in_trade: true,
                ..Default::default()
            },
        ),
        (
            "side_1".to_string(),
            SETTINGS_ORDER_FILTER {
                key: "side".to_string(),
                kwargs_string: MAP::from_iter([("side".to_string(), "buy".to_string())]),
                used_orders: vec!["open_order".to_string()],
                use_in_trade: true,
                ..Default::default()
            },
        ),
        (
            "wrap".to_string(),
            SETTINGS_ORDER_FILTER {
                key: "wrap".to_string(),
                used_orders: vec!["order".to_string()],
                use_in_trade: true,
                ..Default::default()
            },
        ),
    ])
});
