use agent_desktop_core::{
    error::{AdapterError, ErrorCode},
    system::audio::{AudioRequest, AudioState},
};
use std::os::raw::{c_uint, c_void};

type OSStatus = i32;
type AudioObjectID = u32;

#[repr(C)]
struct AudioObjectPropertyAddress {
    selector: c_uint,
    scope: c_uint,
    element: c_uint,
}

const SYSTEM_OBJECT: AudioObjectID = 1;
const DEFAULT_OUTPUT: c_uint = u32::from_be_bytes(*b"dOut");
const VIRTUAL_MAIN_VOLUME: c_uint = u32::from_be_bytes(*b"vmvc");
const VOLUME_SCALAR: c_uint = u32::from_be_bytes(*b"volm");
const MUTE: c_uint = u32::from_be_bytes(*b"mute");
const SCOPE_OUTPUT: c_uint = u32::from_be_bytes(*b"outp");
const SCOPE_GLOBAL: c_uint = u32::from_be_bytes(*b"glob");
const ELEMENT_MAIN: c_uint = 0;
const ELEMENT_CHANNEL_1: c_uint = 1;
const ELEMENT_CHANNEL_2: c_uint = 2;

unsafe extern "C" {
    fn AudioObjectGetPropertyData(
        id: AudioObjectID,
        addr: *const AudioObjectPropertyAddress,
        qual_size: u32,
        qual: *const c_void,
        data_size: *mut u32,
        data: *mut c_void,
    ) -> OSStatus;
    fn AudioObjectSetPropertyData(
        id: AudioObjectID,
        addr: *const AudioObjectPropertyAddress,
        qual_size: u32,
        qual: *const c_void,
        data_size: u32,
        data: *const c_void,
    ) -> OSStatus;
    fn AudioObjectHasProperty(id: AudioObjectID, addr: *const AudioObjectPropertyAddress) -> bool;
}

fn default_output() -> Result<AudioObjectID, AdapterError> {
    let addr = AudioObjectPropertyAddress {
        selector: DEFAULT_OUTPUT,
        scope: SCOPE_GLOBAL,
        element: ELEMENT_MAIN,
    };
    let mut dev: AudioObjectID = 0;
    let mut size = std::mem::size_of::<AudioObjectID>() as u32;
    let st = unsafe {
        AudioObjectGetPropertyData(
            SYSTEM_OBJECT,
            &addr,
            0,
            std::ptr::null(),
            &mut size,
            &mut dev as *mut _ as *mut c_void,
        )
    };
    if st != 0 || dev == 0 {
        return Err(fail("default output device", st));
    }
    Ok(dev)
}

fn has_virtual_volume(dev: AudioObjectID) -> bool {
    has_property(dev, VIRTUAL_MAIN_VOLUME, SCOPE_OUTPUT)
}

fn read_volume_virtual(dev: AudioObjectID) -> Result<u8, AdapterError> {
    let addr = AudioObjectPropertyAddress {
        selector: VIRTUAL_MAIN_VOLUME,
        scope: SCOPE_OUTPUT,
        element: ELEMENT_MAIN,
    };
    let mut v: f32 = 0.0;
    let mut size = std::mem::size_of::<f32>() as u32;
    let st = unsafe {
        AudioObjectGetPropertyData(
            dev,
            &addr,
            0,
            std::ptr::null(),
            &mut size,
            &mut v as *mut _ as *mut c_void,
        )
    };
    if st != 0 {
        return Err(fail("read virtual volume", st));
    }
    Ok((v.clamp(0.0, 1.0) * 100.0).round() as u8)
}

fn write_volume_virtual(dev: AudioObjectID, pct: u8) -> Result<(), AdapterError> {
    let addr = AudioObjectPropertyAddress {
        selector: VIRTUAL_MAIN_VOLUME,
        scope: SCOPE_OUTPUT,
        element: ELEMENT_MAIN,
    };
    let v = f32::from(pct.min(100)) / 100.0;
    let st = unsafe {
        AudioObjectSetPropertyData(
            dev,
            &addr,
            0,
            std::ptr::null(),
            std::mem::size_of::<f32>() as u32,
            &v as *const _ as *const c_void,
        )
    };
    if st != 0 {
        return Err(fail("set virtual volume", st));
    }
    Ok(())
}

fn read_channel_volume(dev: AudioObjectID, element: c_uint) -> Result<f32, AdapterError> {
    let addr = AudioObjectPropertyAddress {
        selector: VOLUME_SCALAR,
        scope: SCOPE_OUTPUT,
        element,
    };
    let mut v: f32 = 0.0;
    let mut size = std::mem::size_of::<f32>() as u32;
    let st = unsafe {
        AudioObjectGetPropertyData(
            dev,
            &addr,
            0,
            std::ptr::null(),
            &mut size,
            &mut v as *mut _ as *mut c_void,
        )
    };
    if st != 0 {
        return Err(fail("read channel volume", st));
    }
    Ok(v.clamp(0.0, 1.0))
}

fn write_channel_volume(dev: AudioObjectID, element: c_uint, v: f32) -> Result<(), AdapterError> {
    let addr = AudioObjectPropertyAddress {
        selector: VOLUME_SCALAR,
        scope: SCOPE_OUTPUT,
        element,
    };
    let st = unsafe {
        AudioObjectSetPropertyData(
            dev,
            &addr,
            0,
            std::ptr::null(),
            std::mem::size_of::<f32>() as u32,
            &v as *const _ as *const c_void,
        )
    };
    if st != 0 {
        return Err(fail("set channel volume", st));
    }
    Ok(())
}

fn read_volume_scalar(dev: AudioObjectID) -> Result<u8, AdapterError> {
    let ch1 = read_channel_volume(dev, ELEMENT_CHANNEL_1)?;
    let ch2 = read_channel_volume(dev, ELEMENT_CHANNEL_2).unwrap_or(ch1);
    Ok(((ch1 + ch2) / 2.0 * 100.0).round() as u8)
}

fn write_volume_scalar(dev: AudioObjectID, pct: u8) -> Result<(), AdapterError> {
    let v = f32::from(pct.min(100)) / 100.0;
    write_channel_volume(dev, ELEMENT_CHANNEL_1, v)?;
    let _ = write_channel_volume(dev, ELEMENT_CHANNEL_2, v);
    Ok(())
}

fn read_volume(dev: AudioObjectID) -> Result<u8, AdapterError> {
    if has_virtual_volume(dev) {
        read_volume_virtual(dev)
    } else {
        read_volume_scalar(dev)
    }
}

fn write_volume(dev: AudioObjectID, pct: u8) -> Result<(), AdapterError> {
    if has_virtual_volume(dev) {
        write_volume_virtual(dev, pct)
    } else {
        write_volume_scalar(dev, pct)
    }
}

fn has_property(dev: AudioObjectID, selector: c_uint, scope: c_uint) -> bool {
    let addr = AudioObjectPropertyAddress {
        selector,
        scope,
        element: ELEMENT_MAIN,
    };
    unsafe { AudioObjectHasProperty(dev, &addr) }
}

fn read_muted(dev: AudioObjectID) -> Result<bool, AdapterError> {
    if !has_property(dev, MUTE, SCOPE_OUTPUT) {
        return Ok(false);
    }
    let addr = AudioObjectPropertyAddress {
        selector: MUTE,
        scope: SCOPE_OUTPUT,
        element: ELEMENT_MAIN,
    };
    let mut m: u32 = 0;
    let mut size = std::mem::size_of::<u32>() as u32;
    let st = unsafe {
        AudioObjectGetPropertyData(
            dev,
            &addr,
            0,
            std::ptr::null(),
            &mut size,
            &mut m as *mut _ as *mut c_void,
        )
    };
    if st != 0 {
        return Err(fail("read mute", st));
    }
    Ok(m != 0)
}

fn write_muted(dev: AudioObjectID, on: bool) -> Result<(), AdapterError> {
    let addr = AudioObjectPropertyAddress {
        selector: MUTE,
        scope: SCOPE_OUTPUT,
        element: ELEMENT_MAIN,
    };
    let m: u32 = u32::from(on);
    let st = unsafe {
        AudioObjectSetPropertyData(
            dev,
            &addr,
            0,
            std::ptr::null(),
            std::mem::size_of::<u32>() as u32,
            &m as *const _ as *const c_void,
        )
    };
    if st != 0 {
        return Err(fail("set mute", st));
    }
    Ok(())
}

fn fail(what: &str, st: OSStatus) -> AdapterError {
    AdapterError::new(ErrorCode::ActionFailed, format!("CoreAudio {what} failed"))
        .with_platform_detail(format!("OSStatus {st}"))
}

pub fn handle(req: AudioRequest) -> Result<AudioState, AdapterError> {
    let dev = default_output()?;
    match req {
        AudioRequest::GetState => {}
        AudioRequest::SetVolume(v) => write_volume(dev, v)?,
        AudioRequest::AdjustVolume(d) => {
            let cur = i16::from(read_volume(dev)?);
            write_volume(dev, (cur + i16::from(d)).clamp(0, 100) as u8)?;
        }
        AudioRequest::SetMuted(on) => write_muted(dev, on)?,
    }
    Ok(AudioState {
        output_volume: read_volume(dev)?,
        muted: read_muted(dev)?,
    })
}
