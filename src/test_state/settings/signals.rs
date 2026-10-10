use crate::test_state::prelude::*;

// these settings need to be checked:
// - one of the kvargs
// - src usage
// - ind usage
// - signals_train usage
// - procedure_used
pub static SIGNALS: LazyLock<SETTINGS_SIGNALS> = LazyLock::new(|| {
    SETTINGS_SIGNALS::from_iter([
        (
            "th_1".to_string(),
            SETTINGS_SIGNAL {
                key: "th".to_string(),
                kwargs: MAP::from_iter([
                    ("index_min".to_string(), from_u128_number(0).unwrap()),
                    ("index_max".to_string(), from_u128_number(0).unwrap()),
                    ("index_normal".to_string(), from_u128_number(0).unwrap()),
                    ("th_min".to_string(),from_f64_number( 0.0001).unwrap()),
                    ("th_max".to_string(),from_f64_number( 0.0001).unwrap()),
                    ("limit".to_string(),from_f64_number( 1.).unwrap()),
                ]),
                used_src: vec![
                    SETTINGS_USED_USIZE {
                        index: 0,
                        sub_from_last_i: 1,
                    },
                    SETTINGS_USED_USIZE {
                        index: 1,
                        sub_from_last_i: 1,
                    },
                ],
                used_ind: vec!["rma_1".to_string()],
                procedure_used_src: vec![1, 0, 2],
                used_signals_train: vec!["mm_1".to_string()],
                ..Default::default()
            },
        ),
        (
            "invert_1".to_string(),
            SETTINGS_SIGNAL {
                key: "invert".to_string(),
                used_signals: vec!["th_1".to_string()],
                ..Default::default()
            },
        ),
        (
            "signal".to_string(),
            SETTINGS_SIGNAL {
                key: "repeat".to_string(),
                kwargs: MAP::from_iter([
                    ("value_signal".to_string(), from_f64_number(1.).unwrap()),
                    ("value_probability".to_string(), from_f64_number(1.).unwrap()),
                ]),
                ..Default::default()
            },
        ),
    ])
});
