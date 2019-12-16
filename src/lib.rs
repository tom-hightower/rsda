#[macro_use]
extern crate vst;
mod helper;

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{CanDo, Category, Info, Plugin};

use helper::{midi_pitch_to_freq, TAU};

struct Rsda {
    sample_rate: f64,
    time: f64,
    note_duration: f64,
    note: Option<u8>,
}

impl Rsda {
    fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate
    }

    fn note_on(&mut self, note: u8) {
        self.note_duration = 0.0;
        self.note = Some(note)
    }

    fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None
        }
    }
}

impl Plugin for Rsda {
    fn get_info(&self) -> Info {
        Info {
            name: "Rsda".to_string(),
            unique_id: 2289,
            inputs: 0,
            category: Category::Synth,
            ..Default::default()
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = f64::from(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        if self.note == None {
            return;
        }
        let samples = buffer.samples();
        let (_, mut output_buffer) = buffer.split();
        let tps = self.time_per_sample();
        let mut output_sample: f32;
        for samp_idx in 0..samples {
            if let Some(current_note) = self.note {
                output_sample = (self.time * midi_pitch_to_freq(current_note) * TAU).sin() as f32;
                self.time += tps;
                self.note_duration += tps;
            } else {
                output_sample = 0.0;
            }
            for buf_idx in 0..output_buffer.len() {
                let buff = output_buffer.get_mut(buf_idx);
                buff[samp_idx] = output_sample;
            }
        }
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => match ev.data[0] {
                    144 => self.note_off(ev.data[1]),
                    128 => self.note_on(ev.data[1]),
                    _ => (),
                },
                _ => (),
            }
        }
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }
}

impl Default for Rsda {
    fn default() -> Rsda {
        Rsda {
            sample_rate: 44100.0,
            note_duration: 0.0,
            time: 0.0,
            note: None,
        }
    }
}

plugin_main!(Rsda);
