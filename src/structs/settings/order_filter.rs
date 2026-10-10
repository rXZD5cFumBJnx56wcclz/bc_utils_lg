use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_ORDER_FILTER {
    pub key: String,
    pub kwargs: MAP<String, serde_json::Value>,
    pub used_orders: Vec<String>,
    pub used_orders_filtered: Vec<String>,
    pub procedure_used_orders: Vec<usize>,
    pub used_src: Vec<SETTINGS_USED_USIZE>,
    pub used_ind: Vec<String>,
    pub used_utils_state: Vec<String>,
    pub use_in_trade: bool,
    // does not apply to bf
    pub procedure_used_src: Vec<usize>,
    pub used_signals: Vec<String>,
}

pub type SETTINGS_ORDER_FILTERS = MAP_LINK<String, SETTINGS_ORDER_FILTER>;