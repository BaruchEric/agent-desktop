use std::sync::Mutex;

use crate::adapter::PlatformAdapter;
use crate::error::AdapterError;
use crate::system::{
    appearance::{AppearanceRequest, AppearanceState},
    audio::{AudioRequest, AudioState},
    controller::SystemController,
    external::{ExternalRequest, ExternalResult},
    network::{NetworkRequest, NetworkState},
};

pub(crate) struct MockSystemController {
    pub volume: Mutex<u8>,
    pub dark: Mutex<bool>,
    pub wifi_power: Mutex<bool>,
    pub external_result: Mutex<ExternalResult>,
    pub last_audio: Mutex<Option<AudioRequest>>,
    pub last_external: Mutex<Option<ExternalRequest>>,
}

impl Default for MockSystemController {
    fn default() -> Self {
        Self {
            volume: Mutex::new(0),
            dark: Mutex::new(false),
            wifi_power: Mutex::new(true),
            external_result: Mutex::new(ExternalResult {
                exit_code: 0,
                stdout: String::new(),
                stderr: String::new(),
                duration_ms: 0,
            }),
            last_audio: Mutex::new(None),
            last_external: Mutex::new(None),
        }
    }
}

#[allow(dead_code)]
impl MockSystemController {
    pub fn last_audio(&self) -> Option<AudioRequest> {
        self.last_audio.lock().unwrap().clone()
    }

    pub fn last_external(&self) -> Option<ExternalRequest> {
        self.last_external.lock().unwrap().clone()
    }

    pub fn external_result(&self, result: ExternalResult) {
        *self.external_result.lock().unwrap() = result;
    }

    pub fn with_volume(volume: u8) -> Self {
        Self {
            volume: Mutex::new(volume),
            ..Default::default()
        }
    }
}

impl SystemController for MockSystemController {
    fn audio(&self, req: AudioRequest) -> Result<AudioState, AdapterError> {
        *self.last_audio.lock().unwrap() = Some(req.clone());
        let mut vol = self.volume.lock().unwrap();
        match req {
            AudioRequest::SetVolume(v) => *vol = v,
            AudioRequest::AdjustVolume(d) => {
                *vol = (i16::from(*vol) + i16::from(d)).clamp(0, 100) as u8;
            }
            _ => {}
        }
        Ok(AudioState {
            output_volume: *vol,
            muted: false,
        })
    }

    fn appearance(&self, req: AppearanceRequest) -> Result<AppearanceState, AdapterError> {
        let mut dark = self.dark.lock().unwrap();
        match req {
            AppearanceRequest::SetDark(v) => *dark = v,
            AppearanceRequest::Toggle => *dark = !*dark,
            AppearanceRequest::Get => {}
        }
        Ok(AppearanceState { dark: *dark })
    }

    fn network(&self, req: NetworkRequest) -> Result<NetworkState, AdapterError> {
        let mut power = self.wifi_power.lock().unwrap();
        if let NetworkRequest::WifiPower(v) = req {
            *power = v;
        }
        Ok(NetworkState {
            wifi_power: *power,
            ssid: None,
        })
    }

    fn run_external(&self, req: ExternalRequest) -> Result<ExternalResult, AdapterError> {
        *self.last_external.lock().unwrap() = Some(req);
        Ok(self.external_result.lock().unwrap().clone())
    }
}

#[derive(Default)]
pub(crate) struct MockSystemAdapter {
    pub system: MockSystemController,
}

impl PlatformAdapter for MockSystemAdapter {
    fn system(&self) -> &dyn SystemController {
        &self.system
    }
}

#[cfg(test)]
mod selfcheck {
    use super::*;
    use crate::adapter::PlatformAdapter;
    use crate::system::audio::AudioRequest;

    #[test]
    fn adapter_routes_to_mock_controller() {
        let adapter = MockSystemAdapter::default();
        let state = adapter.system().audio(AudioRequest::SetVolume(42)).unwrap();
        assert_eq!(state.output_volume, 42);
        assert!(matches!(
            adapter.system.last_audio().unwrap(),
            AudioRequest::SetVolume(42)
        ));
    }
}
