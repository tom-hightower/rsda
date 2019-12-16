use std::f64::consts::PI;

const A4_PITCH: i8 = 69;
const A4_FREQ: f64 = 440.0;
pub const TAU: f64 = PI * 2.0;

pub fn midi_pitch_to_freq(pitch: u8) -> f64 {
    ((f64::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}
