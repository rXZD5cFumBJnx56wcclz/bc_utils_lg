use crate::test_state::prelude::*;

// these settings need to be checked:
// - market
// - limit
// - trigger
// signal used
// utils state used
pub static ORDER_CREATORS: LazyLock<SettingsOrderCreators> = LazyLock::new(|| {
    SettingsOrderCreators::from_iter([
        (
            "open_order".to_string(),
            SettingsOrderCreator {
                type_: "market".to_string(),
                used_signal: "th_1".to_string(),
                used_util_state: "qty_1".to_string(),
                leverage: 10.,
                ..Default::default()
            },
        ),
        (
            "avg_order".to_string(),
            SettingsOrderCreator {
                type_: "limit".to_string(),
                used_signal: "th_1".to_string(),
                used_util_state: "qty_1".to_string(),
                used_ind: Some("sma_1".to_string()),
                leverage: 10.,
                ..Default::default()
            },
        ),
        (
            "sl".to_string(),
            SettingsOrderCreator {
                type_: "market".to_string(),
                used_signal: "th_1".to_string(),
                include_in_storage: true,
                used_util_state: "qty_1".to_string(),
                leverage: 10.,
                trigger: Some(SettingsTriggerOutOfStorage {
                    used_ind: "rma_1".to_string(),
                    trigger_by: "last".to_string(),
                    used_util_state: "direction_1".to_string(),
                }),
                is_reduce: true,
                ..Default::default()
            },
        ),
        (
            "order".to_string(),
            SettingsOrderCreator {
                type_: "market".to_string(),
                used_signal: "signal".to_string(),
                used_util_state: "qty_1".to_string(),
                leverage: 10.,
                ..Default::default()
            },
        ),
    ])
});
