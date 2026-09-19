use crate::test_state::prelude::*;

pub static INDICATIONS_COLUMNS: LazyLock<SETTINGS_INDS> =
    LazyLock::new(|| SETTINGS_INDS::default());
