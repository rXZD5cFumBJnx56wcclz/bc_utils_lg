use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
// src, post data, ind_col data, ind_val data
pub struct SettingsInd {
    pub key: String,
    pub kwargs: MAP<String, serde_json::Value>,
    pub used_src: Vec<SettingsUsedUsize>,
    pub used_ind: Vec<String>,
    pub procedure_used: Vec<usize>,
    // pipeline post only
    pub used_database: Vec<SettingsUsedString>,
    pub used_ind_col: Vec<SettingsUsedString>,
    pub used_ind_val: Vec<String>,
}
pub type SettingsInds = MAP_LINK<String, SettingsInd>;
