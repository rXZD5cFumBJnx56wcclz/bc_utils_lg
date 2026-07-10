#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;
use std::{error::Error, path::PathBuf};

use serde_json5::from_reader;

use crate::types::maps::{MAP, MAP_LINK};

pub fn settings_from_json(dir: PathBuf) -> Result<SETTINGS, Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(dir)?);
    from_reader(&mut reader).map_err(|e| Box::new(e) as Box<dyn Error>)
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_USED_STRING_USIZE {
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
pub struct SETTINGS_IND {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_STRING_USIZE>,
    pub used_ind: Vec<String>,
    pub procedure_used: Vec<usize>,
}
pub type SETTINGS_INDS = MAP_LINK<String, SETTINGS_IND>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_SIGNAL {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_STRING_USIZE>,
    pub used_ind: Vec<String>,
    pub used_signals: Vec<String>,
    pub procedure_used_src: Vec<usize>,
    pub procedure_used_signals: Vec<usize>,
}
pub type SETTINGS_SIGNALS = MAP_LINK<String, SETTINGS_SIGNAL>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_EXCH {
    pub url: String,
    pub key: String,
    pub secret: String,
    pub exchange: String,
    pub timeout_req_ms: usize,
    pub timeout_cycle_ms: usize,
}

impl Default for SETTINGS_EXCH {
    fn default() -> Self {
        Self {
            url: Default::default(),
            key: Default::default(),
            secret: Default::default(),
            exchange: Default::default(),
            timeout_req_ms: 5000,
            timeout_cycle_ms: 7000,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_ORDER_COLLECTOR {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    // (1: key, 2: key_ind)
    pub used_signals_ready: Vec<(String, String)>,
}
pub type SETTINGS_ORDER_COLLECTORS = Vec<SETTINGS_ORDER_COLLECTOR>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_SYMBOL_FILTER {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_STRING_USIZE>,
    pub used_ind: Vec<SETTINGS_USED_STRING>,
    pub used_ind_stat_columns: Vec<SETTINGS_USED_STRING>,
    pub used_ind_stat_values: Vec<String>,
    pub procedure_used_src: Vec<usize>,
}
pub type SETTINGS_SYMBOL_FILTERS = Vec<SETTINGS_SYMBOL_FILTER>;

// fix
// configuration for creating SL/TP orders if the position has not been created
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SETTINGS_CREATE_TP_SL_ORDERS {
    pub tp_market: bool,
    pub tp_limit: bool,
    pub tp_trigger_market: bool,
    pub tp_trigger_limit: bool,
    pub sl_market: bool,
    pub sl_limit: bool,
    pub sl_trigger_market: bool,
    pub sl_trigger_limit: bool,
}

impl Default for SETTINGS_CREATE_TP_SL_ORDERS {
    fn default() -> Self {
        Self {
            tp_market: true,
            sl_market: true,
            tp_limit: false,
            tp_trigger_market: false,
            tp_trigger_limit: false,
            sl_limit: false,
            sl_trigger_market: false,
            sl_trigger_limit: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_TRADE {
    pub signal_hold: f64,
    pub signal_short: f64,
    pub signal_long: f64,
    pub commission_market: f64,
    pub commission_limit: f64,
    pub capital: f64,
    pub percent_of_capital: f64,
    pub amount_of_capital: f64,
    pub max_entry: usize,
    pub max_exit: usize,
    pub market_mult_of_probability_qty: f64,
    pub limit_mult_of_probability_qty: f64,
    pub market_entry_orders_signals: Vec<String>,
    pub market_exit_orders_signals: Vec<String>,
    pub limit_entry_orders_signals: Vec<(String, String)>,
    pub limit_exit_orders_signals: Vec<(String, String)>,
    pub trigger_market_entry_orders_signals: Vec<(String, String)>,
    pub trigger_market_exit_orders_signals: Vec<(String, String)>,
    pub trigger_limit_entry_orders_signals: Vec<(String, String, String)>,
    pub trigger_limit_exit_orders_signals: Vec<(String, String, String)>,
    pub create_tp_sl_orders: SETTINGS_CREATE_TP_SL_ORDERS,
    pub order_collectors: SETTINGS_ORDER_COLLECTORS,
    pub stoploss: Vec<(f64, f64, f64)>,
    pub takeprofit: Vec<(f64, f64, f64)>,
    pub trigger_by: String,
    pub work_in_real_time: bool,
    pub category: String,
    pub account_type: String,
    pub klines_qty: usize,
    pub timeframe: String,
    pub leverage: f64,
    pub mode_trade: String,
    pub hedge_mode: bool,
    pub symbols_filters: Option<SETTINGS_SYMBOL_FILTERS>,
    pub symbols_time_update_ms: usize,
    pub symbols: Vec<String>,
    pub symbols_black_list: Vec<String>,
    pub coins: Vec<String>,
    pub coins_black_list: Vec<String>,
    pub slippage_tolerance_type: String,
    pub slippage_tolerance: (f64, f64),
    pub time_in_force: String,
}

impl Default for SETTINGS_TRADE {
    fn default() -> Self {
        Self {
            signal_hold: 0.,
            signal_short: -1.,
            signal_long: 1.,
            commission_market: 0.001,
            commission_limit: 0.001,
            capital: 1000.,
            percent_of_capital: 0.01,
            amount_of_capital: 0.,
            max_entry: usize::MAX,
            max_exit: usize::MAX,
            market_mult_of_probability_qty: 1.,
            limit_mult_of_probability_qty: 1.,
            market_entry_orders_signals: Default::default(),
            market_exit_orders_signals: Default::default(),
            limit_entry_orders_signals: Default::default(),
            limit_exit_orders_signals: Default::default(),
            trigger_market_entry_orders_signals: Default::default(),
            trigger_market_exit_orders_signals: Default::default(),
            trigger_limit_entry_orders_signals: Default::default(),
            trigger_limit_exit_orders_signals: Default::default(),
            create_tp_sl_orders: Default::default(),
            order_collectors: vec![SETTINGS_ORDER_COLLECTOR {
                key: "clear".to_string(),
                ..Default::default()
            }],
            stoploss: Default::default(),
            takeprofit: Default::default(),
            trigger_by: "last".to_string(),
            work_in_real_time: false,
            category: "linear".to_string(),
            account_type: "UNIFIED".to_string(),
            klines_qty: Default::default(),
            timeframe: "1".to_string(),
            leverage: 1.0,
            mode_trade: "isolated".to_string(),
            hedge_mode: true,
            symbols_filters: None,
            symbols_time_update_ms: 60 * 60 * 24 * 1000,
            symbols: Default::default(),
            symbols_black_list: Default::default(),
            coins: Default::default(),
            coins_black_list: Default::default(),
            slippage_tolerance_type: Default::default(),
            slippage_tolerance: Default::default(),
            time_in_force: "GTC".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_FILES_DIR {
    pub script_backtest: PathBuf,
    pub script_stat: PathBuf,
    pub backtest: PathBuf,
    pub src: PathBuf,
    pub train_model: PathBuf,
}

impl Default for SETTINGS_FILES_DIR {
    fn default() -> Self {
        Self {
            script_backtest: Default::default(),
            script_stat: Default::default(),
            // /23_00_24_24_06_2026/report.html
            // /23_00_24_24_06_2026/SUIUSDT/data.dat
            // /23_00_24_24_06_2026/SUIUSDT/stat_values.dat
            // /23_00_24_24_06_2026/SUIUSDT/stat_columns.dat
            // /23_00_24_24_06_2026/SUIUSDT/script_data.plt
            // /23_00_24_24_06_2026/SUIUSDT/script_stat.plt
            // /23_00_24_24_06_2026/SUIUSDT/backtest.svg
            // /23_00_24_24_06_2026/SUIUSDT/capital.svg
            // /23_00_24_24_06_2026/SUIUSDT/stat.svg
            backtest: "target/bc_constructor/backtests".into(),
            src: "target/bc_constructor/data".into(),
            train_model: "target/bc_constructor/train_models".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS_MSG {
    pub key: String,
    pub chat: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SETTINGS {
    pub exch: SETTINGS_EXCH,
    pub indications: SETTINGS_INDS,
    pub signals_train: SETTINGS_SIGNALS,
    pub signals_ready: SETTINGS_SIGNALS,
    pub trade: SETTINGS_TRADE,
    pub files_dir: SETTINGS_FILES_DIR,
    pub indications_stat_values: SETTINGS_INDS,
    pub indications_stat_columns: SETTINGS_INDS,
}
