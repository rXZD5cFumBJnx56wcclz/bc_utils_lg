use crate::test_state::prelude::*;

pub static GLOBAL: LazyLock<SETTINGS_GLOBAL> = LazyLock::new(|| SETTINGS_GLOBAL {
    exch: EXCH.clone(),
    files_dir: FILES_DIR.clone(),
    trade: TRADE.clone(),
    data_gen: Some(DATA_GEN.clone()),
    other: OTHER.clone(),
});
