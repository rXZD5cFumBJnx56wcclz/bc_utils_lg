use crate::prelude::*;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SettingsExch {
    pub url: String,
    pub wws_host: String,
    pub wws_url: String,
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

impl Default for SettingsExch {
    fn default() -> Self {
        Self {
            url: Default::default(),
            wws_host: Default::default(),
            wws_url: Default::default(),
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
