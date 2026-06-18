use clap::Parser;
use serde::Deserialize;

fn default_step() -> u8 {
    5
}

#[derive(Parser, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VolumeArgs {
    #[arg(long, help = "Print current output volume and mute state")]
    #[serde(default)]
    pub get: bool,
    #[arg(long, value_name = "N", help = "Set output volume 0..=100")]
    pub set: Option<u8>,
    #[arg(long, help = "Raise volume by --step")]
    #[serde(default)]
    pub up: bool,
    #[arg(long, help = "Lower volume by --step")]
    #[serde(default)]
    pub down: bool,
    #[arg(long, help = "Mute output")]
    #[serde(default)]
    pub mute: bool,
    #[arg(long, help = "Unmute output")]
    #[serde(default)]
    pub unmute: bool,
    #[arg(long, default_value = "5", help = "Step size for --up/--down")]
    #[serde(default = "default_step")]
    pub step: u8,
}
