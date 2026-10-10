use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
// src, post data, ind_col data, ind_val data
pub struct SETTINGS_IND {
    pub key: String,
    pub kwargs: MAP<String, serde_json::Value>,
    pub used_src: Vec<SETTINGS_USED_USIZE>,
    pub used_ind: Vec<String>,
    pub procedure_used: Vec<usize>,
    // pipeline post only
    pub used_database: Vec<SETTINGS_USED_STRING>,
    pub used_ind_col: Vec<SETTINGS_USED_STRING>,
    pub used_ind_val: Vec<String>,
}
pub type SETTINGS_INDS = MAP_LINK<String, SETTINGS_IND>;