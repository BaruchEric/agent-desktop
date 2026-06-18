use crate::error::AdapterError;
use crate::system::{
    appearance::{AppearanceRequest, AppearanceState},
    audio::{AudioRequest, AudioState},
    external::{ExternalRequest, ExternalResult},
    network::{NetworkRequest, NetworkState},
};

pub trait SystemController: Send + Sync {
    fn audio(&self, _req: AudioRequest) -> Result<AudioState, AdapterError> {
        Err(AdapterError::not_supported("system.audio"))
    }

    fn appearance(&self, _req: AppearanceRequest) -> Result<AppearanceState, AdapterError> {
        Err(AdapterError::not_supported("system.appearance"))
    }

    fn network(&self, _req: NetworkRequest) -> Result<NetworkState, AdapterError> {
        Err(AdapterError::not_supported("system.network"))
    }

    fn run_external(&self, _req: ExternalRequest) -> Result<ExternalResult, AdapterError> {
        Err(AdapterError::not_supported("system.run_external"))
    }
}

pub struct UnsupportedSystemController;

impl SystemController for UnsupportedSystemController {}

pub static UNSUPPORTED_SYSTEM: UnsupportedSystemController = UnsupportedSystemController;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::PlatformAdapter;
    use crate::error::ErrorCode;

    #[test]
    fn unsupported_controller_returns_not_supported() {
        let c = UnsupportedSystemController;
        let err = c
            .audio(crate::system::audio::AudioRequest::GetState)
            .unwrap_err();
        assert_eq!(err.code, ErrorCode::PlatformNotSupported);
    }

    #[test]
    fn default_adapter_system_is_unsupported() {
        struct Bare;
        impl PlatformAdapter for Bare {}
        let bare = Bare;
        let err = bare
            .system()
            .appearance(crate::system::appearance::AppearanceRequest::Get)
            .unwrap_err();
        assert_eq!(err.code, ErrorCode::PlatformNotSupported);
    }
}
