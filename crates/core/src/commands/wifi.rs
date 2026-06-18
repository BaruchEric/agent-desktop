use crate::{adapter::PlatformAdapter, error::AppError, system::network::NetworkRequest};
use serde_json::{Value, json};

pub struct WifiArgs {
    pub on: bool,
    pub off: bool,
    pub status: bool,
}

pub fn execute(args: WifiArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    let selected = [args.on, args.off, args.status]
        .iter()
        .filter(|x| **x)
        .count();
    if selected != 1 {
        return Err(AppError::invalid_input(
            "wifi needs exactly one of --on/--off/--status",
        ));
    }
    let req = if args.status {
        NetworkRequest::WifiStatus
    } else {
        NetworkRequest::WifiPower(args.on)
    };
    let state = adapter.system().network(req)?;
    let mut data = json!({ "wifi_power": state.wifi_power });
    if let Some(ssid) = state.ssid {
        data["ssid"] = json!(ssid);
    }
    Ok(data)
}

#[cfg(test)]
#[path = "wifi_tests.rs"]
mod tests;
