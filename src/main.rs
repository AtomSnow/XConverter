mod benchmark;
mod render;
mod realtime;
mod rpc;
mod playsound;

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
                    playsound::playsound("./assets/render-on.wav");
                    render::render(&self.midipath, &self.sfzpath);
                }
                if ui.button("Play").clicked() {
                    playsound::playsound("./assets/render-on.wav");
                    realtime::play(&self.midipath, &self.sfzpath);
                }
                if ui.button("Benchmark").clicked() {
                    //playsound();
                    benchmark::benchmark();
                    playsound::playsound("./assets/render-on.wav");
                }
            });
        }
    }



