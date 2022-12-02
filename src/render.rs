use std::{
    time::Instant,
};

use xsynth_render::builder::{xsynth_renderer, XSynthRenderStats};

pub fn render(midi: &str, strsfz: &str, layersc: &str){

    let layers: usize = layersc.parse().unwrap();
    let mut sfz = Vec::new();
    sfz.push(strsfz);

    let out = "output.wav";
    let render_time = Instant::now();

    let callback = |stats: XSynthRenderStats| {
        print!("\rMIDI position: {}", stats.progress);
    };

    xsynth_renderer(midi, out)
        .with_config(Default::default())
        .add_soundfonts(sfz)
        .with_layer_count(Some(layers))
        .with_progress_callback(callback)
        .run();

    print!("\x1B[2J\x1B[1;1H");
    println!("Render time: {} seconds", render_time.elapsed().as_secs());
}