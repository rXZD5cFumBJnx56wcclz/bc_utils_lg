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
            kwargs: MAP::from_iter([
                ("index_min".to_string(), from_u128_number(0).unwrap()),
                ("index_max".to_string(), from_u128_number(0).unwrap()),
                ("min_distance".to_string(), from_u128_number(3).unwrap()),
                ("window".to_string(), from_u128_number(5).unwrap()),
                ("tp_th".to_string(), from_f64_number(0.0001).unwrap()),
                ("tp_limit".to_string(), from_f64_number(0.01).unwrap()),
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
