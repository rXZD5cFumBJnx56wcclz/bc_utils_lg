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

pub fn from_json<T: DeserializeOwned>(dir: PathBuf) -> Result<T, Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(dir)?);
    from_reader(&mut reader).map_err(|e| Box::new(e) as Box<dyn Error>)
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_EXCH {
    pub url: String,
    pub wws_host: String,
    pub wws_url: String,
    pub key: String,
    pub secret: String,
    pub exchange: String,
    pub account_type: String,
    #[serde_as(as = "DurationSeconds<u64>")]
    pub timeframe_sec: Duration,
    pub category: String,
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub timeout_req_ms: Duration,
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub timeout_cycle_ms: Duration,
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub ping_ms: Duration,
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub timeout_socket_ms: Duration,
}

impl Default for SETTINGS_EXCH {
    fn default() -> Self {
        Self {
            url: Default::default(),
            wws_host: Default::default(),
            wws_url: Default::default(),
            key: Default::default(),
            secret: Default::default(),
            exchange: Default::default(),
            account_type: "UNIFIED".to_string(),
            timeframe_sec: Duration::from_mins(1),
            category: "linear".to_string(),
            timeout_req_ms: Duration::from_secs(5),
            timeout_cycle_ms: Duration::from_secs(7),
            ping_ms: Duration::from_secs(10),
            timeout_socket_ms: Duration::from_secs(1),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, transparent)]
pub struct SETTINGS_FILES_DIR(pub PathBuf);

impl Default for SETTINGS_FILES_DIR {
    fn default() -> Self {
        Self("target/bc_constructor".into())
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_USED_USIZE {
    pub index: usize,
    pub sub_from_last_i: usize,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_USED_STRING {
    pub key: String,
    pub sub_from_last_i: usize,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_GW {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
}

pub type SETTINGS_GW_MAP = MAP<String, SETTINGS_GW>;

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
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_USIZE>,
    pub used_ind: Vec<SETTINGS_USED_STRING>,
    pub used_ind_columns: Vec<SETTINGS_USED_STRING>,
    pub used_ind_values: Vec<String>,
    pub procedure_used_src: Vec<usize>,
}
pub type SETTINGS_SYMBOL_FILTERS = Vec<SETTINGS_SYMBOL_FILTER>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
// src, post data, ind_col data, ind_val data
pub struct SETTINGS_IND {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_USIZE>,
    pub used_ind: Vec<String>,
    pub procedure_used: Vec<usize>,
    // pipeline post only
    pub used_database: Vec<SETTINGS_USED_STRING>,
    pub used_ind_col: Vec<SETTINGS_USED_STRING>,
    pub used_ind_val: Vec<String>,
}
pub type SETTINGS_INDS = MAP_LINK<String, SETTINGS_IND>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_SIGNAL {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_USIZE>,
    pub used_ind: Vec<String>,
    pub used_signals_train: Vec<String>,
    pub procedure_used_src: Vec<usize>,
    pub used_signals: Vec<String>,
}
pub type SETTINGS_SIGNALS = MAP_LINK<String, SETTINGS_SIGNAL>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_TRIGGER_OUT_OF_STORAGE {
    pub used_ind: String,
    pub trigger_by: String,
    // trigger direction
    pub used_util_state: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_ORDER_CREATOR {
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
    pub trigger: Option<SETTINGS_TRIGGER_OUT_OF_STORAGE>,
    pub type_price_cross: String,
    pub leverage: f64,
}

impl Default for SETTINGS_ORDER_CREATOR {
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

pub type SETTINGS_ORDER_CREATORS = MAP<String, SETTINGS_ORDER_CREATOR>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_ORDER_FILTER {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
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

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_ORDER_COLLECTOR {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    // (1: key, 2: key_ind)
    pub used_signals: Vec<String>,
    pub used_ind: Vec<String>,
}
pub type SETTINGS_ORDER_COLLECTORS = Vec<SETTINGS_ORDER_COLLECTOR>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_UTIL_STATE {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_USIZE>,
    pub used_ind: Vec<String>,
    pub procedure_used_src: Vec<usize>,
    pub used_signals: Vec<String>,
}

pub type SETTINGS_UTILS_STATE = MAP<String, SETTINGS_UTIL_STATE>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_TRADE {
    pub capital: f64,
    pub work_in_real_time: bool,
    pub klines_qty: usize,
    pub leverage: f64,
    pub mode_trade: String,
    pub mode_hedge: bool,
    pub slippage_tolerance_type: String,
    pub slippage_tolerance: (f64, f64),
    pub time_in_force: String,
    pub commission_market: f64,
    pub commission_limit: f64,
    pub signal_hold: f64,
    pub signal_long: f64,
    pub signal_short: f64,
}

impl Default for SETTINGS_TRADE {
    fn default() -> Self {
        Self {
            capital: 1000.,
            work_in_real_time: false,
            klines_qty: Default::default(),
            leverage: 1.0,
            mode_trade: "isolated".to_string(),
            mode_hedge: true,
            slippage_tolerance_type: Default::default(),
            slippage_tolerance: Default::default(),
            time_in_force: "GTC".to_string(),
            commission_market: 0.001,
            commission_limit: 0.001,
            signal_hold: 0.,
            signal_long: 1.,
            signal_short: -1.,
        }
    }
}

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
