use crate::test_state::prelude::*;

pub static PIPELINE: LazyLock<SETTINGS_PIPELINE> = LazyLock::new(|| SETTINGS_PIPELINE {
    indications: INDICATIONS.clone(),
    signals_train: SIGNALS_TRAIN.clone(),
    signals: SIGNALS.clone(),
    utils_state: UTILS_STATE.clone(),
    order_creators: ORDER_CREATORS.clone(),
    order_filters: ORDER_FILTERS.clone(),
    order_collectors: ORDER_COLLECTORS.clone(),
});
