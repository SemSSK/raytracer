mod camera;
mod math;
mod vec3;

use std::{f32::consts::PI, time::Instant};

use camera::CameraTransform;
use egui::{Color32, ColorImage, TextureHandle, Ui};
use math::{get_vector_from_index, Collidable, Ray, Sphere};
use nalgebra::Rotation3;
use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::*;
use vec3::{ConvertableToColor, Vec3};

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
    camera: (Vec3, Rotation3<f32>),
    pub camera_transform: CameraTransform,
    pub sphere: Sphere,
    pub light_direction: Vec3,
    pixels: Vec<Color32>,
    time: f32,
}

impl MyApp {
    fn render(&mut self, ui: &mut Ui) {
        self.camera = self.camera_transform.update(&self.camera);
        let now = Instant::now();
        let pixels = self
            .pixels
            .par_iter()
            .enumerate()
            .map(|(i, _)| {
                let viewport_pos = get_vector_from_index(i, self.width, self.height, &self.camera);
                let col = Color32::BLACK;
                let ray = Ray {
                    position: self.camera.0,
                    direction: viewport_pos - self.camera.0,
                };
                match self
                    .sphere
                    .find_color_to_display(&ray, &self.light_direction)
                {
                    Some(c) => c.as_color(),
                    None => col,
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
            camera: (
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: -5.,
                },
                Rotation3::identity(),
            ),
            camera_transform: Default::default(),
            light_direction: Vec3 {
                x: -1.,
                y: -1.,
                z: -1.,
            }
            .normalized(),
            sphere: Sphere {
                color: Vec3 {
                    x: 0.75,
                    y: 0.66,
                    z: 0.45,
                },
                ray: 0.5,
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
                        ui.label("Sphere ray");
                        ui.add(egui::DragValue::new(&mut self.sphere.ray));
                        ui.end_row();
                        ui.label("Sphere x position");
                        ui.add(egui::DragValue::new(&mut self.sphere.center.x).speed(0.1));
                        ui.end_row();
                        ui.label("Sphere y position");
                        ui.add(egui::DragValue::new(&mut self.sphere.center.y).speed(0.1));
                        ui.end_row();
                        ui.label("Sphere z position");
                        ui.add(egui::DragValue::new(&mut self.sphere.center.z).speed(0.1));
                        ui.end_row();
                        ui.label("Light x position");
                        ui.add(egui::DragValue::new(&mut self.light_direction.x).speed(0.1));
                        ui.end_row();
                        ui.label("Light y position");
                        ui.add(egui::DragValue::new(&mut self.light_direction.y).speed(0.1));
                        ui.end_row();
                        ui.label("Light z position");
                        ui.add(egui::DragValue::new(&mut self.light_direction.z).speed(0.1));
                        ui.end_row();
                        ui.label("Camera x rotation");
                        ui.add(
                            egui::Slider::new(
                                &mut self.camera_transform.rot_x,
                                (-PI / 2.)..=(PI / 2.),
                            )
                            .step_by(0.05),
                        );
                        ui.end_row();
                        ui.label("Camera y rotation");
                        ui.add(
                            egui::Slider::new(
                                &mut self.camera_transform.rot_y,
                                (-PI / 2.)..=(PI / 2.),
                            )
                            .step_by(0.05),
                        );
                        ui.end_row();
                        ui.label("Camera z rotation");
                        ui.add(
                            egui::Slider::new(
                                &mut self.camera_transform.rot_z,
                                (-PI / 2.)..=(PI / 2.),
                            )
                            .step_by(0.05),
                        );
                        ui.end_row();
                        ui.label("Camera x position");
                        ui.add(egui::DragValue::new(&mut self.camera_transform.trans_x).speed(0.1));
                        ui.end_row();
                        ui.label("Camera y position");
                        ui.add(egui::DragValue::new(&mut self.camera_transform.trans_y).speed(0.1));
                        ui.end_row();
                        ui.label("Camera z position");
                        ui.add(egui::DragValue::new(&mut self.camera_transform.trans_z).speed(0.1));
                        ui.end_row();
                    });
                // if ui.button("Render 🎥").clicked() {
                self.render(ui);
                // }
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
