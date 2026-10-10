use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_ORDER_COLLECTOR {
    pub key: String,
    pub kwargs: MAP<String, serde_json::Value>,
    // (1: key, 2: key_ind)
    pub used_signals: Vec<String>,
    pub used_ind: Vec<String>,
}
pub type SETTINGS_ORDER_COLLECTORS = Vec<SETTINGS_ORDER_COLLECTOR>;
