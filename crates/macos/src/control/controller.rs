use agent_desktop_core::{
    error::AdapterError,
    system::{
        appearance::{AppearanceRequest, AppearanceState},
        audio::{AudioRequest, AudioState},
        controller::SystemController,
    },
};

pub struct MacSystemController;

impl MacSystemController {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MacSystemController {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemController for MacSystemController {
    fn audio(&self, req: AudioRequest) -> Result<AudioState, AdapterError> {
        crate::control::audio::handle(req)
    }

    fn appearance(&self, req: AppearanceRequest) -> Result<AppearanceState, AdapterError> {
        crate::control::appearance::handle(req)
    }
}
