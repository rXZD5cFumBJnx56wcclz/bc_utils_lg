use crate::test_state::prelude::*;

// fix
// Add shared functions for everyone to use.
pub static UTILS_STATE: LazyLock<SettingsUtilsStates> = LazyLock::new(|| {
    SettingsUtilsStates::from_iter([
        (
            "qty_1".to_string(),
            SettingsUtilStates {
                key: "qty".to_string(),
                kwargs: MAP::from_iter([
                    ("amount_usd".to_string(), from_f64_number(1.).unwrap()),
                    (
                        "percent_of_capital".to_string(),
                        from_f64_number(0.1).unwrap(),
                    ),
                ]),
                ..Default::default()
            },
        ),
        (
            "direction_1".to_string(),
            SettingsUtilStates {
                key: "direction".to_string(),
                kwargs: MAP::from_iter([("direction".to_string(), from_f64_number(1.).unwrap())]),
                ..Default::default()
            },
        ),
    ])
});
