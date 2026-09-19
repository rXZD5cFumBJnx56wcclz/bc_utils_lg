use crate::test_state::prelude::*;

// these settings need to be checked:
// - one of the kvargs
// - src usage
// - ind usage

// without verification
// - signals_train usage
// - procedure_used
pub static SIGNALS_TRAIN: LazyLock<SETTINGS_SIGNALS> = LazyLock::new(|| {
    SETTINGS_SIGNALS::from_iter([(
        "mm_1".to_string(),
        SETTINGS_SIGNAL {
            key: "mm".to_string(),
            kwargs_usize: MAP::from_iter([
                ("index_min".to_string(), 0),
                ("index_max".to_string(), 0),
                ("min_distance".to_string(), 3),
                ("window".to_string(), 5),
            ]),
            kwargs_f64: MAP::from_iter([
                ("tp_th".to_string(), 0.0001),
                ("tp_limit".to_string(), 0.01),
            ]),
            used_src: vec![SETTINGS_USED_USIZE {
                index: 1,
                sub_from_last_i: 1,
            }],
            used_ind: vec!["rma_1".to_string()],
            ..Default::default()
        },
    )])
});
