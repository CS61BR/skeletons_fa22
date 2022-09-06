use wasm_bindgen::prelude::*;

use crate::gh2::{guitarstring::GuitarString, random::Random};
const NUM_STRINGS: usize = 128;

#[wasm_bindgen]
extern "C" {
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log_str(s: &str);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn console_log_str(s: &str) {
    println!("{}", s);
}

macro_rules! log {
    ($($t:tt)*) => (console_log_str(&format_args!($($t)*).to_string()))
}
pub(crate) use log; // make log macro public

#[wasm_bindgen]
pub struct MusicBox {
    samples_returned: usize,
    sample_rate: u32,
    random: Random,
    player: Player,
}

enum Player {
    Midi {
        index: usize,
        samples_per_tick: f64,
        notes: Vec<Note>,
        strings: Vec<GuitarString>,
        volumes: Vec<f32>,
    },
    SingleNote(GuitarString),
}

#[derive(Debug)]
struct Note {
    time: u32, // in ticks
    on: bool,
    key: u8,
    vel: u8,
}

const MIDI_BYTES: &[u8; 58896] = include_bytes!("../../ttfaf");
const MIDI_MPT: f64 = 1171.875;

#[wasm_bindgen]
impl MusicBox {
    pub fn new(sample_rate: u32, note_atten: f32, is_ttaf: bool, freq: u32) -> Self {
        log!("new music box created!");
        let player = if is_ttaf {
            let mut strings = Vec::new();
            let mut volumes = Vec::new();
            for i in 0..NUM_STRINGS {
                strings.push(GuitarString::new(
                    sample_rate,
                    note_atten,
                    (440.0 * ((i as f64 - 69.0) / 12.0).exp2()) as u32,
                ));
                volumes.push(0.);
            }

            let notes = bytes_to_notes(MIDI_BYTES);
            let samples_per_tick = (sample_rate as f64 / 1e6) * MIDI_MPT;

            Player::Midi {
                index: 0,
                samples_per_tick,
                notes,
                strings,
                volumes,
            }
        } else {
            Player::SingleNote(GuitarString::new(sample_rate, note_atten, freq))
        };

        Self {
            samples_returned: 0,
            sample_rate,
            random: Random::new("cheese"),
            player,
        }
    }

    pub fn process(&mut self, cheese: &mut [f32]) -> bool {

        match &mut self.player {
            Player::SingleNote(gs) => {
                if self.samples_returned == 0 {
                    gs.pluck(&mut self.random);
                }

                for sample in cheese.iter_mut() {
                    *sample = gs.advance();
                }

                self.samples_returned += cheese.len();
                // play note for 3 seconds
                self.samples_returned < (3 * self.sample_rate as usize)
            }
            Player::Midi {
                index,
                samples_per_tick,
                notes,
                strings,
                volumes,
            } => {

                for (a, sample) in cheese.iter_mut().enumerate() {
                    while *index < notes.len()
                        && which_sample(notes[*index].time, *samples_per_tick)
                            <= (self.samples_returned + a)
                    {
                        let note = notes[*index].key as usize;
                        if notes[*index].on {
                            volumes[note] = notes[*index].vel as f32 / 127.;
                            strings[note].pluck(&mut self.random);
                        } else {
                            volumes[note] = 0.0; // bad, should ramp down. Oh well
                        }
                        *index += 1;
                    }

                    let mut sum = 0.0;
                    for i in 0..NUM_STRINGS {
                        sum += strings[i].advance() * volumes[i];
                    }
                    *sample = sum;
                }

                self.samples_returned += cheese.len();
                // last note + 3 seconds
                self.samples_returned
                    < (3 * self.sample_rate as usize)
                        + which_sample(notes.last().unwrap().time, *samples_per_tick)
            }
        }
    }
}

fn which_sample(time: u32, samples_per_tick: f64) -> usize {
    (time as f64 * samples_per_tick) as usize
}

fn bytes_to_notes(bytes: &[u8]) -> Vec<Note> {
    let mut res = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let time = (bytes[i] as u32) | (bytes[i+1] as u32) << 8 | (bytes[i+2] as u32) << 16 | (bytes[i+3] as u32) << 24;
        let (on, key) = if bytes[i+4] >= 128 {
            (true, bytes[i+4] - 128)
        } else {
            (false, bytes[i+4])
        };
        let vel = bytes[i+5];
        res.push(Note{ time, on, key, vel});
        i += 6;
    }
    res
}
