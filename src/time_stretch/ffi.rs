use std::os::raw::c_char;
use std::{ffi::c_void, fmt::Write};

#[cfg(debug_assertions)]
use super::bindings::*;

#[cfg(not(debug_assertions))]
use soundtouch_sys as bindings;

type SamplesRead = u32;
struct SoundTouch {
    core: bindings::soundtouch_SoundTouch,
}

impl SoundTouch {
    pub fn new(channels: u32, sample_rate: u32) -> Self {
        unsafe {
            let mut s = bindings::soundtouch_SoundTouch::new();
            bindings::soundtouch_SoundTouch_setChannels(
                &mut s as *mut soundtouch_SoundTouch,
                channels as u32,
            );
            bindings::soundtouch_SoundTouch_setSampleRate(
                &mut s as *mut soundtouch_SoundTouch,
                sample_rate,
            );
            Self { core: s }
        }
    }

    pub fn get_version_string() -> String {
        let mut s = String::new();
        unsafe {
            let c = bindings::soundtouch_SoundTouch_getVersionString() as *const c_char;
            let c_str = std::ffi::CStr::from_ptr(c);
            s = c_str.to_string_lossy().to_owned();
        }
        s
    }

    pub fn get_version_id() -> u32 {
        unsafe { bindings::soundtouch_SoundTouch_getVersionId() }
    }

    pub fn set_rate(&mut self, rate: f64) {
        unsafe {
            bindings::soundtouch_SoundTouch_setRate(
                &mut self.core as *mut soundtouch_SoundTouch,
                rate,
            );
        }
    }

    pub fn set_tempo(&mut self, tempo: f64) {
        unsafe {
            bindings::soundtouch_SoundTouch_setTempo(
                &mut self.core as *mut soundtouch_SoundTouch,
                tempo,
            )
        }
    }

    pub fn set_pitch(&mut self, pitch: f64) {
        unsafe {
            bindings::soundtouch_SoundTouch_setPitch(
                &mut self.core as *mut soundtouch_SoundTouch,
                pitch,
            )
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            self.core.flush();
        }
    }

    pub fn put_samples(&mut self, samples: &[i16]) {
        unsafe {
            let num_samples = samples.len() as u32;
            bindings::soundtouch_SoundTouch_putSamples(
                &mut self.core as *mut soundtouch_SoundTouch as *mut c_void,
                samples.as_ptr(),
                num_samples,
            );
        }
    }

    pub fn receive_samples(
        &mut self,
        output_buf: &mut Vec<i16>,
        max_samples: usize,
    ) -> SamplesRead {
        if output_buf.len() < max_samples {
            output_buf.resize(max_samples, 0);
        }
        unsafe {
            bindings::soundtouch_SoundTouch_receiveSamples(
                &mut self.core as *mut soundtouch_SoundTouch as *mut c_void,
                output_buf.as_mut_ptr(),
                max_samples as u32,
            )
        }
    }
}

impl Drop for SoundTouch {
    fn drop(&mut self) {
        unsafe {
            bindings::soundtouch_SoundTouch_SoundTouch_destructor(
                &mut self.core as *mut soundtouch_SoundTouch,
            );
        }
    }
}
