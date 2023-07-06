mod math;
mod vec3;

use std::time::Instant;

use egui::{Color32, ColorImage, TextureHandle, Ui};
use math::{get_vector_from_index, Collidable, Ray, Sphere};
use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::*;
use vec3::Vec3;

const WINDOW_DIMENSIONS: (f32, f32) = (1366., 768.);

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
    render: Option<TextureHandle>,
    width: usize,
    height: usize,
    sphere: Sphere,
    pixels: Vec<Color32>,
    time: f32,
}

impl MyApp {
    fn render(&mut self, ui: &mut Ui) {
        let now = Instant::now();
        self.pixels.par_iter_mut().for_each(|x| {
            let mut rng = rand::thread_rng();
            *x = Color32::from_rgb(rng.gen(), rng.gen(), rng.gen());
        });
        let pixels = self
            .pixels
            .par_iter()
            .enumerate()
            .map(|(i, _)| {
                let ray = Ray {
                    position: get_vector_from_index(i),
                    direction: Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 1.,
                    },
                };
                if self.sphere.find_if_collides(&ray) {
                    Color32::RED
                } else {
                    Color32::LIGHT_BLUE
                }
            })
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
        let pixels = (0..(600 * 800))
            .into_iter()
            .map(|_| Color32::WHITE)
            .collect::<Vec<_>>();
        Self {
            render: Default::default(),
            width: 800,
            height: 600,
            pixels,
            time: 0.0,
            sphere: Sphere {
                ray: 20.,
                center: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 5.,
                },
            },
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right(egui::Id::new("right panel"))
            .min_width(WINDOW_DIMENSIONS.0 / 4.)
            .show(ctx, |ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.colored_label(Color32::LIGHT_BLUE, "Informations");
                        ui.end_row();
                        ui.label("Time to render in seconds");
                        ui.label(format!("{}", self.time));
                        ui.end_row();
                        ui.label("frames per second (fps)");
                        ui.label(format!("{}", 1. / self.time));
                        ui.end_row();
                        ui.colored_label(Color32::LIGHT_GREEN, "Commands");
                        ui.end_row();
                    });
                if ui.button("Render 🎥").clicked() {
                    self.render(ui);
                }
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| match &self.render {
                    Some(image) => ui.image(image, image.size_vec2()),
                    None => ui.label("No image to render"),
                },
            )
        });
    }
}
