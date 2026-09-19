use crate::test_state::prelude::*;

pub static ORDER_COLLECTORS: LazyLock<SETTINGS_ORDER_COLLECTORS> = LazyLock::new(|| {
    SETTINGS_ORDER_COLLECTORS::from_iter([SETTINGS_ORDER_COLLECTOR {
        key: "clear".to_string(),
        ..Default::default()
    }])
});
