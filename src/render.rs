use std::{
    time::Instant,
};

use xsynth_render::{xsynth_renderer};

pub fn render(midi: &str, strsfz: &str){

    let mut sfz = Vec::new();
    sfz.push(strsfz);

    let out = "output.wav";
    let render_time = Instant::now();

    xsynth_renderer(midi, out)
        .with_config(Default::default())
        .add_soundfonts(sfz)
        .with_layer_count(Some(10))
        .run();

    println!("Render time: {} seconds", render_time.elapsed().as_secs());
}