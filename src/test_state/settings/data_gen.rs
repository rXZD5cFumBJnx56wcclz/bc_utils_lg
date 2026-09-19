use crate::test_state::prelude::*;

pub static DATA_GEN: LazyLock<SETTINGS_DATA_GEN> = LazyLock::new(|| SETTINGS_DATA_GEN {
    fullness_init: "order_collectors".to_string(),
    fullness_step: "order_filters".to_string(),
    fullness_execute: "order_collectors".to_string(),
});
