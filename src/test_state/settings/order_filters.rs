use crate::test_state::prelude::*;

// fix
pub static ORDER_FILTERS: LazyLock<SettingsOrderFilters> = LazyLock::new(|| {
    SettingsOrderFilters::from_iter([
        (
            "count_1".to_string(),
            SettingsOrderFilter {
                key: "count".to_string(),
                kwargs: MAP::from_iter([("max_count".to_string(), from_u128_number(1).unwrap())]),
                used_orders: vec!["open_order".to_string()],
                ..Default::default()
            },
        ),
        (
            "count_2".to_string(),
            SettingsOrderFilter {
                key: "count".to_string(),
                kwargs: MAP::from_iter([("max_count".to_string(), from_u128_number(1).unwrap())]),
                used_orders: vec!["avg_order".to_string()],
                use_in_trade: true,
                ..Default::default()
            },
        ),
        (
            "count_3".to_string(),
            SettingsOrderFilter {
                key: "count".to_string(),
                kwargs: MAP::from_iter([("max_count".to_string(), from_u128_number(1).unwrap())]),
                used_orders: vec!["sl".to_string()],
                use_in_trade: true,
                ..Default::default()
            },
        ),
        (
            "side_1".to_string(),
            SettingsOrderFilter {
                key: "side".to_string(),
                kwargs: MAP::from_iter([("side".to_string(), Value::String("buy".to_string()))]),
                used_orders: vec!["open_order".to_string()],
                use_in_trade: true,
                ..Default::default()
            },
        ),
        (
            "wrap".to_string(),
            SettingsOrderFilter {
                key: "wrap".to_string(),
                used_orders: vec!["order".to_string()],
                use_in_trade: true,
                ..Default::default()
            },
        ),
    ])
});
