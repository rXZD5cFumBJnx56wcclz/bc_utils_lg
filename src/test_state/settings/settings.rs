use crate::test_state::prelude::*;

pub static SETTINGS_ALL: LazyLock<SETTINGS> = LazyLock::new(|| SETTINGS {
    global: GLOBAL.clone(),
    symbols_all: SYMBOLS_ALL.clone(),
    pipeline: PIPELINE.clone(),
    pipeline_post: PIPELINE_POST.clone(),
});
