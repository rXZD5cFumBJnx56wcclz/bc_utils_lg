use crate::test_state::prelude::*;

pub static FILES_DIR: LazyLock<SETTINGS_FILES_DIR> =
    LazyLock::new(|| SETTINGS_FILES_DIR("target/bc_constructor".into()));
