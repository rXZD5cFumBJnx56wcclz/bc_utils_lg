#![allow(non_camel_case_types)]

pub use std::fs::File;
pub use std::io::BufReader;
pub use std::{error::Error, path::PathBuf};

use serde::de::DeserializeOwned;

use crate::prelude::*;

pub fn from_json<T: DeserializeOwned>(dir: PathBuf) -> Result<T, Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(dir)?);
    from_reader(&mut reader).map_err(|e| Box::new(e) as Box<dyn Error>)
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SettingsUsedUsize {
    pub index: usize,
    pub sub_from_last_i: usize,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SettingsUsedString {
    pub key: String,
    pub sub_from_last_i: usize,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SettingsGw {
    pub key: String,
    pub kwargs: MAP<String, serde_json::Value>,
}

pub type SettingsGwMap = MAP<String, SettingsGw>;
