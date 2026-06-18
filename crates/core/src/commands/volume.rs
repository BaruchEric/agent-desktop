use crate::{
    adapter::PlatformAdapter, commands::helpers::exactly_one, error::AppError,
    system::audio::AudioRequest,
};
use serde_json::{Value, json};

pub struct VolumeArgs {
    pub get: bool,
    pub set: Option<u8>,
    pub up: bool,
    pub down: bool,
    pub mute: bool,
    pub unmute: bool,
    pub step: u8,
}

pub fn execute(args: VolumeArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    let req = to_request(&args)?;
    let state = adapter.system().audio(req)?;
    Ok(json!({ "output_volume": state.output_volume, "muted": state.muted }))
}

fn to_request(args: &VolumeArgs) -> Result<AudioRequest, AppError> {
    if !exactly_one(&[
        args.get,
        args.set.is_some(),
        args.up,
        args.down,
        args.mute,
        args.unmute,
    ]) {
        return Err(AppError::invalid_input(
            "volume needs exactly one of --get/--set/--up/--down/--mute/--unmute",
        ));
    }
    if let Some(v) = args.set {
        if v > 100 {
            return Err(AppError::invalid_input("--set must be 0..=100"));
        }
        return Ok(AudioRequest::SetVolume(v));
    }
    if args.get {
        return Ok(AudioRequest::GetState);
    }
    if args.up {
        return Ok(AudioRequest::AdjustVolume(
            i8::try_from(args.step).unwrap_or(i8::MAX),
        ));
    }
    if args.down {
        return Ok(AudioRequest::AdjustVolume(
            -i8::try_from(args.step).unwrap_or(i8::MAX),
        ));
    }
    Ok(AudioRequest::SetMuted(args.mute))
}

#[cfg(test)]
#[path = "volume_tests.rs"]
mod tests;
