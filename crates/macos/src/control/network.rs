use agent_desktop_core::{
    error::{AdapterError, ErrorCode},
    system::network::{NetworkRequest, NetworkState},
};
use std::process::Command;
use std::time::Duration;

const NETWORKSETUP_TIMEOUT: Duration = Duration::from_secs(10);

fn run(args: &[&str]) -> Result<String, AdapterError> {
    let mut cmd = Command::new("networksetup");
    cmd.args(args);
    let out =
        crate::system::process::run_with_timeout(&mut cmd, "networksetup", NETWORKSETUP_TIMEOUT)?;
    if !out.status.success() {
        return Err(
            AdapterError::new(ErrorCode::ActionFailed, "networksetup failed")
                .with_platform_detail(String::from_utf8_lossy(&out.stderr).to_string()),
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

fn wifi_device() -> Result<String, AdapterError> {
    let listing = run(&["-listallhardwareports"])?;
    let mut lines = listing.lines();
    while let Some(line) = lines.next() {
        if line.contains("Wi-Fi") || line.contains("AirPort") {
            for next in lines.by_ref() {
                if let Some(dev) = next.strip_prefix("Device: ") {
                    return Ok(dev.trim().to_string());
                }
                if next.trim().is_empty() {
                    break;
                }
            }
        }
    }
    Err(AdapterError::new(
        ErrorCode::ActionFailed,
        "no Wi-Fi device found",
    ))
}

fn power_on(dev: &str) -> Result<bool, AdapterError> {
    let out = run(&["-getairportpower", dev])?;
    Ok(out.trim().ends_with("On"))
}

pub fn handle(req: NetworkRequest) -> Result<NetworkState, AdapterError> {
    let dev = wifi_device()?;
    match req {
        NetworkRequest::WifiStatus => {}
        NetworkRequest::WifiPower(on) => {
            run(&["-setairportpower", &dev, if on { "on" } else { "off" }])?;
        }
    }
    Ok(NetworkState {
        wifi_power: power_on(&dev)?,
        ssid: None,
    })
}
