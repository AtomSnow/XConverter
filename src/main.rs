mod benchmark;
mod render;
mod realtime;
//mod rpc;
mod playsound;

use eframe::egui;
//use egui::ProgressBar;
use std::thread;
//use std::time::Duration;

//# [tokio::main]
fn main() {
    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        min_window_size: Some(egui::vec2(1024.0, 512.0)),
        max_window_size: Some(egui::vec2(1024.0, 512.0)),
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
    layers: String,
//    pb: f32,
}

impl Default for Xcv {
    fn default() -> Self {
        Self {
            midipath: "".to_owned(),
            sfzpath: "".to_owned(),
            layers: "10".to_owned(),
//            pb: 42.0,
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
                ui.label("SFZ Path: ");
                ui.text_edit_singleline(&mut self.sfzpath);
                ui.label("Layer Count: ");
                ui.text_edit_singleline(&mut self.layers);
            });
                if ui.button("Render")
                .clicked()
                {
                    thread::spawn(|| {
                        playsound::playsound("./assets/render-on.wav");
                    });
                    render::render(&self.midipath, &self.sfzpath, &self.layers);
                }
                if ui.button("Play")
                .clicked() 
                {
                    thread::spawn(|| {
                        playsound::playsound("./assets/render-on.wav");
                    });
                    realtime::play(&self.midipath, &self.sfzpath);
                }
                if ui.button("Benchmark")
                .clicked() 
                {
                    thread::spawn(|| {
                        playsound::playsound("./assets/render-on.wav");
                    });
                    benchmark::benchmark();
                }
                //ui.spacing_mut().slider_width = ctx.available_rect().width() - 20.0;
                //let progress = *scalar / 360.0;
                //ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                //    ui.add(egui::ProgressBar::show_percentage(progress));
                //});
        });
    }
}



