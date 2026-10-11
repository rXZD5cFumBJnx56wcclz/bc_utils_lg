use crate::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SettingsSignal {
    pub key: String,
    pub kwargs: MAP<String, serde_json::Value>,
    pub used_src: Vec<SettingsUsedUsize>,
    pub used_ind: Vec<String>,
    pub used_signals_train: Vec<String>,
    pub procedure_used_src: Vec<usize>,
    pub used_signals: Vec<String>,
}
pub type SettingsSignals = MAP_LINK<String, SettingsSignal>;
