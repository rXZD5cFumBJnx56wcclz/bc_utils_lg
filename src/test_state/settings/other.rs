use crate::test_state::prelude::*;

pub static OTHER: LazyLock<SETTINGS_OTHER> = LazyLock::new(|| SETTINGS_OTHER {
    reboot_ms: Some(Duration::from_hours(24)),
});
