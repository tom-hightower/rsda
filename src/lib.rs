#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin, Category};

#[derive(Default)]
struct Rsda;

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
}

plugin_main!(Rsda);