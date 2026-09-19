use crate::test_state::prelude::*;

pub static INDICATIONS_VALUES: LazyLock<SETTINGS_INDS> = LazyLock::new(|| {
    SETTINGS_INDS::from_iter([(
        "profit_factor_1".to_string(),
        SETTINGS_IND {
            key: "profit_factor".to_string(),
            used_ind: vec!["capital".to_string()],
            ..Default::default()
        },
    )])
});
