use std::time::Instant;

use egui::{Color32, ColorImage, TextureHandle, Ui};
use rand::{rngs::ThreadRng, Rng};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2 { x: 1366., y: 768. }),
        ..Default::default()
    };
    eframe::run_native(
        "Raytracer",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    rng: ThreadRng,
    render: Option<TextureHandle>,
    width: usize,
    height: usize,
    pixels: Vec<u8>,
    time: f32,
}

impl MyApp {
    fn render(&mut self, ui: &mut Ui) {
        let now = Instant::now();
        self.pixels.iter_mut().for_each(|x| *x = self.rng.gen());
        let pixels = self
            .pixels
            .chunks_exact(3)
            .map(|p| Color32::from_rgb(p[0], p[1], p[2]))
            .collect::<Vec<_>>();
        self.render = Some(ui.ctx().load_texture(
            "render",
            ColorImage {
                pixels,
                size: [self.width, self.height],
            },
            Default::default(),
        ));
        self.time = now.elapsed().as_secs_f32();
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let pixels = (0..(600 * 800 * 3))
            .into_iter()
            .map(|_| 255)
            .collect::<Vec<_>>();
        Self {
            rng: rand::thread_rng(),
            render: Default::default(),
            width: 800,
            height: 600,
            pixels,
            time: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right(egui::Id::new("right panel")).show(ctx, |ui| {
            ui.label(format!("render time {} seconds", self.time));
            if ui.button("Render").clicked() {
                self.render(ui);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| match &self.render {
                    Some(image) => ui.image(image, image.size_vec2()),
                    None => ui.label("No image to render"),
                },
            )
        });
    }
}
