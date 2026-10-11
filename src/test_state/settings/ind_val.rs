use crate::test_state::prelude::*;

pub static INDICATIONS_VALUES: LazyLock<SettingsInds> = LazyLock::new(|| {
    SettingsInds::from_iter([(
        "profit_factor_1".to_string(),
        SettingsInd {
            key: "profit_factor".to_string(),
            used_ind: vec!["capital".to_string()],
            ..Default::default()
        },
    )])
});
