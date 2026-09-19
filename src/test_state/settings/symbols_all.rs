use crate::test_state::prelude::*;

pub static SYMBOLS_ALL: LazyLock<SETTINGS_SYMBOLS_ALL> = LazyLock::new(|| SETTINGS_SYMBOLS_ALL {
    symbols: SYMBOLS.clone(),
    symbols_filters: SYMBOLS_FILTERS.clone(),
});
