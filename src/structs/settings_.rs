#![allow(non_camel_case_types)]

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_with::{DurationMilliSeconds, DurationSeconds, serde_as};

use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::{error::Error, path::PathBuf};

use serde_json5::from_reader;

use crate::types::maps::{MAP, MAP_LINK};




#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_SYMBOLS {
    pub symbols: Option<Vec<String>>,
    pub coins: Option<Vec<String>>,
    pub symbols_black_list: Vec<String>,
    pub coins_black_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_SYMBOL_FILTER {
    pub key: String,
    pub kwargs: MAP<String, serde_json::Value>,
    pub used_src: Vec<SETTINGS_USED_USIZE>,
    pub used_ind: Vec<SETTINGS_USED_STRING>,
    pub used_ind_columns: Vec<SETTINGS_USED_STRING>,
    pub used_ind_values: Vec<String>,
    pub procedure_used_src: Vec<usize>,
}
pub type SETTINGS_SYMBOL_FILTERS = Vec<SETTINGS_SYMBOL_FILTER>;







#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_OTHER {
    pub reboot_ms: Option<Duration>,
}

impl Default for SETTINGS_OTHER {
    fn default() -> Self {
        Self {
            reboot_ms: Some(Duration::from_hours(24)),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_DATA_GEN {
    pub fullness_init: String,
    pub fullness_step: String,
    pub use_trade_state: bool,
    pub fullness_execute: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_GLOBAL {
    pub exch: SETTINGS_EXCH,
    pub files_dir: SETTINGS_FILES_DIR,
    pub trade: SETTINGS_TRADE,
    pub data_gen: Option<SETTINGS_DATA_GEN>,
    pub other: SETTINGS_OTHER,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_SYMBOLS_ALL {
    pub symbols: SETTINGS_SYMBOLS,
    pub symbols_filters: SETTINGS_SYMBOL_FILTERS,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_PIPELINE {
    pub indications: SETTINGS_INDS,
    pub signals_train: SETTINGS_SIGNALS,
    pub signals: SETTINGS_SIGNALS,
    pub utils_state: SETTINGS_UTILS_STATE,
    pub order_creators: SETTINGS_ORDER_CREATORS,
    pub order_filters: SETTINGS_ORDER_FILTERS,
    pub order_collectors: SETTINGS_ORDER_COLLECTORS,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_PIPELINE_POST {
    pub indications_columns: SETTINGS_INDS,
    pub indications_values: SETTINGS_INDS,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS {
    pub global: SETTINGS_GLOBAL,
    pub symbols_all: SETTINGS_SYMBOLS_ALL,
    pub pipeline: SETTINGS_PIPELINE,
    pub pipeline_post: SETTINGS_PIPELINE_POST,
}
