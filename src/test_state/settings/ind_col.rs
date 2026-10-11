use crate::test_state::prelude::*;

pub static INDICATIONS_COLUMNS: LazyLock<SettingsInds> =
    LazyLock::new(|| SettingsInds::default());
