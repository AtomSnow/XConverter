mod benchmark;
mod render;
mod realtime;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use std::thread;

//# [tokio::main]
fn main() {
    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        min_window_size: Some(egui::vec2(512.0, 256.0)),
        max_window_size: Some(egui::vec2(512.0, 256.0)),
        resizable: false,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        fullscreen: false,
        ..Default::default()
    };
    eframe::run_native(
        "XConverter",
        options,
        Box::new(|_cc| Box::new(Xcv::default())),
    );
}

struct Xcv {
    midipath: String,
    sfzpath: String,
}

impl Default for Xcv {
    fn default() -> Self {
        Self {
            midipath: "".to_owned(),
            sfzpath: "".to_owned(),
        }
    }
}
impl eframe::App for Xcv {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("XConverter");
            ui.horizontal(|ui| {
                ui.label("MIDI Path: ");
                ui.text_edit_singleline(&mut self.midipath);
            });
            ui.horizontal(|ui| {
                ui.label("SFZ Path: ");
                ui.text_edit_singleline(&mut self.sfzpath);
            });
                if ui.button("Render").clicked() {
                    //playsound();
                    render::render(&self.midipath, &self.sfzpath);
                }
                if ui.button("Play").clicked() {
                    //playsound();
                    realtime::play(&self.midipath, &self.sfzpath);
                }
                if ui.button("Benchmark").clicked() {
                    //playsound();
                    benchmark::benchmark();
                    playsound("./assets/check-on.wav");
                }
            });
        }
    }


fn playsound(soundtype: &str){ 
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open(soundtype).unwrap());
        // ./assets/check-on.wav
        // ./assets/check-off.wav
        let source = Decoder::new(file).unwrap();
        stream_handle.play_raw(source.convert_samples());
        std::thread::sleep(std::time::Duration::from_secs(3));
}