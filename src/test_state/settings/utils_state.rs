use crate::test_state::prelude::*;

// fix
// Add shared functions for everyone to use.
pub static UTILS_STATE: LazyLock<SETTINGS_UTILS_STATE> = LazyLock::new(|| {
    SETTINGS_UTILS_STATE::from_iter([
        (
            "qty_1".to_string(),
            SETTINGS_UTIL_STATE {
                key: "qty".to_string(),
                kwargs_f64: MAP::from_iter([
                    ("amount_usd".to_string(), 1.),
                    ("percent_of_capital".to_string(), 0.1),
                ]),
                ..Default::default()
            },
        ),
        (
            "direction_1".to_string(),
            SETTINGS_UTIL_STATE {
                key: "direction".to_string(),
                kwargs_f64: MAP::from_iter([("direction".to_string(), 1.)]),
                ..Default::default()
            },
        ),
    ])
});
