use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioRequest {
    GetState,
    SetVolume(u8),
    AdjustVolume(i8),
    SetMuted(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioState {
    pub output_volume: u8,
    pub muted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_state_roundtrips() {
        let s = AudioState {
            output_volume: 50,
            muted: false,
        };
        let v = serde_json::to_value(&s).unwrap();
        assert_eq!(v["output_volume"], 50);
        assert_eq!(v["muted"], false);
    }
}
