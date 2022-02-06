extern crate sdl2;

use sdl2::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::time::Duration;

struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;
            self.pos += 1;
        }
    }
}

fn main() -> Result<(), String> {
    let wav_file: Cow<'static, Path> = match std::env::args().nth(1) {
        None => Cow::from(Path::new("random.wav")),
        Some(s) => Cow::from(PathBuf::from(s)),
    };
    let sdl_context = sdl2::init()?;
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1), // mono
        samples: None,     // default
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav(wav_file).expect("Could not load test WAV file");

        let cvt = AudioCVT::new(
            wav.format,
            wav.channels,
            wav.freq,
            spec.format,
            spec.channels,
            spec.freq,
        )
        .expect("Could not convert WAV file");

        let data = cvt.convert(wav.buffer().to_vec());

        Sound {
            data: data,
            volume: 0.25,
            pos: 0,
        }
    })?;

    device.resume();

    std::thread::sleep(Duration::from_millis(3_000));

    Ok(())
}