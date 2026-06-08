use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde_json::from_reader;

use crate::structs::settings::SETTINGS;

pub fn settings_from_json(dir: &str) -> Result<SETTINGS, Box<dyn Error>> {
    let reader = BufReader::new(File::open(dir)?);
    from_reader(reader).map_err(|e| Box::new(e) as Box<dyn Error>)
}
