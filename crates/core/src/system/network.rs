use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkRequest {
    WifiStatus,
    WifiPower(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkState {
    pub wifi_power: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_state_omits_ssid_when_none() {
        let s = NetworkState {
            wifi_power: true,
            ssid: None,
        };
        let v = serde_json::to_value(&s).unwrap();
        assert!(!v.as_object().unwrap().contains_key("ssid"));
    }
}
