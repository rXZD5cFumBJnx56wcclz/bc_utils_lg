use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SettingsTriggerOutOfStorage {
    pub used_ind: String,
    pub trigger_by: String,
    // trigger direction
    pub used_util_state: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SettingsOrderCreator {
    // market/limit
    pub type_: String,
    // side
    pub used_signal: String,
    // qty
    pub used_util_state: String,
    pub is_reduce: bool,
    // limit price
    pub used_ind: Option<String>,
    // if order is trigger
    pub include_in_storage: bool,
    // if order is trigger
    pub trigger: Option<SettingsTriggerOutOfStorage>,
    pub type_price_cross: String,
    pub leverage: f64,
}

impl Default for SettingsOrderCreator {
    fn default() -> Self {
        Self {
            type_: Default::default(),
            used_signal: Default::default(),
            used_util_state: Default::default(),
            is_reduce: Default::default(),
            used_ind: Default::default(),
            include_in_storage: Default::default(),
            trigger: Default::default(),
            type_price_cross: "last".to_string(),
            leverage: 1.,
        }
    }
}

pub type SettingsOrderCreators = MAP<String, SettingsOrderCreator>;
