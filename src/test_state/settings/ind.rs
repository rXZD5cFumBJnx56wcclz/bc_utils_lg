use crate::test_state::prelude::*;

// these settings need to be checked:
// - one of the kvargs
// - src usage
// - ind usage
// - procedure_used
pub static INDICATIONS: LazyLock<SETTINGS_INDS> = LazyLock::new(|| {
    SETTINGS_INDS::from_iter([
        (
            "rma_1".to_string(),
            SETTINGS_IND {
                key: "rma".to_string(),
                kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
                used_src: vec![SETTINGS_USED_USIZE {
                    index: 4,
                    sub_from_last_i: 1,
                }],
                ..Default::default()
            },
        ),
        (
            "sma_1".to_string(),
            SETTINGS_IND {
                key: "sma".to_string(),
                kwargs_usize: MAP::from_iter([("window".to_string(), 3)]),
                used_src: vec![
                    // fake
                    SETTINGS_USED_USIZE {
                        index: 0,
                        ..Default::default()
                    },
                ],
                used_ind: vec!["rma_1".to_string()],
                procedure_used: vec![1, 0],
                ..Default::default()
            },
        ),
    ])
});

pub static INDICATIONS_RSI: LazyLock<SETTINGS_INDS> = LazyLock::new(|| {
    SETTINGS_INDS::from_iter([(
        "rsi_1".to_string(),
        SETTINGS_IND {
            key: "rsi".to_string(),
            kwargs_usize: MAP::from_iter([("window".to_string(), 2)]),
            used_src: vec![SETTINGS_USED_USIZE {
                index: 1,
                ..Default::default()
            }],
            ..Default::default()
        },
    )])
});
