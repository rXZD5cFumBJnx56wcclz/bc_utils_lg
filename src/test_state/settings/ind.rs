use crate::test_state::prelude::*;

// these settings need to be checked:
// - one of the kvargs
// - src usage
// - ind usage
// - procedure_used
pub static INDICATIONS: LazyLock<SettingsInds> = LazyLock::new(|| {
    SettingsInds::from_iter([
        (
            "rma_1".to_string(),
            SettingsInd {
                key: "rma".to_string(),
                kwargs: MAP::from_iter([("window".to_string(), from_u128_number(2).unwrap())]),
                used_src: vec![SettingsUsedUsize {
                    index: 4,
                    sub_from_last_i: 1,
                }],
                ..Default::default()
            },
        ),
        (
            "sma_1".to_string(),
            SettingsInd {
                key: "sma".to_string(),
                kwargs: MAP::from_iter([("window".to_string(), from_u128_number(3).unwrap())]),
                used_src: vec![
                    // fake
                    SettingsUsedUsize {
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

pub static INDICATIONS_RSI: LazyLock<SettingsInds> = LazyLock::new(|| {
    SettingsInds::from_iter([(
        "rsi_1".to_string(),
        SettingsInd {
            key: "rsi".to_string(),
            kwargs: MAP::from_iter([("window".to_string(), from_u128_number(2).unwrap())]),
            used_src: vec![SettingsUsedUsize {
                index: 1,
                ..Default::default()
            }],
            ..Default::default()
        },
    )])
});
