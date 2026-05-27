#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::fmt::Debug as Debugg;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debugg)]
pub struct RESULT_FUNDUNG_HISTORY1
{
    pub symbol: String,
    pub fundingRate: String,
    pub fundingRateTimestamp: String,
}

#[derive(Serialize, Deserialize, Debugg)]
pub struct RESULT_FUNDING_HISTORY
{
    pub category: String,
    pub list: Vec<RESULT_FUNDUNG_HISTORY1>
}
