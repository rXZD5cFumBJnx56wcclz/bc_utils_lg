#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

use crate::types::maps::{MAP, MAP_LINK};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SETTINGS_USED_SRC {
    pub index: usize,
    pub sub_from_last_i: usize,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SETTINGS_IND {
    pub key: String,
    pub kwargs_usize: MAP<String, usize>,
    pub kwargs_f64: MAP<String, f64>,
    pub kwargs_string: MAP<String, String>,
    pub used_src: Vec<SETTINGS_USED_SRC>,
    pub used_ind: Vec<String>,
    pub order_used: Vec<usize>,
}
pub type SETTINGS_INDS = MAP_LINK<String, SETTINGS_IND>;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SETTINGS_EXCH {
    pub url: String,
    pub key: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SETTINGS_MSG {
    pub key: String,
    pub chat: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SETTINGS {
    pub exch: SETTINGS_EXCH,
    pub indications: SETTINGS_INDS,
}
