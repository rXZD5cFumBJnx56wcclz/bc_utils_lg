use crate::test_state::prelude::*;

pub static SYMBOLS_FILTERS: LazyLock<SETTINGS_SYMBOL_FILTERS> = LazyLock::new(|| {
    SETTINGS_SYMBOL_FILTERS::from_iter([SETTINGS_SYMBOL_FILTER {
        key: "ordering".to_string(),
        kwargs_f64: MAP::from_iter([("value".to_string(), 1.3)]),
        kwargs_string: MAP::from_iter([("type_".to_string(), "greater".to_string())]),
        used_ind_values: vec!["profit_factor_1".to_string()],
        ..Default::default()
    }])
});
