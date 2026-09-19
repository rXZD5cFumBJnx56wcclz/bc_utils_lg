use crate::test_state::prelude::*;

pub static PIPELINE_POST: LazyLock<SETTINGS_PIPELINE_POST> =
    LazyLock::new(|| SETTINGS_PIPELINE_POST {
        indications_values: INDICATIONS_VALUES.clone(),
        indications_columns: INDICATIONS_COLUMNS.clone(),
    });
