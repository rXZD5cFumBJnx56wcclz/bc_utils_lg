use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_UTIL_STATE {
    pub key: String,
    pub kwargs: MAP<String, serde_json::Value>,
    pub used_src: Vec<SETTINGS_USED_USIZE>,
    pub used_ind: Vec<String>,
    pub procedure_used_src: Vec<usize>,
    pub used_signals: Vec<String>,
}

pub type SETTINGS_UTILS_STATE = MAP<String, SETTINGS_UTIL_STATE>;