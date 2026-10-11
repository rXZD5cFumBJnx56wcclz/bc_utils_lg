use crate::test_state::prelude::*;

pub static ORDER_COLLECTORS: LazyLock<SettingsOrderCollectors> = LazyLock::new(|| {
    SettingsOrderCollectors::from_iter([SettingsOrderCollector {
        key: "clear".to_string(),
        ..Default::default()
    }])
});
