mod camera;
mod math;
mod vec3;

use std::time::Instant;

use camera::CameraTransform;
use egui::{Color32, ColorImage, TextureHandle, Ui, Visuals};
use math::{get_vector_from_index, Ray, Sphere};
use nalgebra::{Rotation3, Vector3};
use vec3::ConvertableToColor;

#[cfg(not(target_arch = "wasm32"))]

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2 { x: 1366., y: 768. }),
        min_window_size: Some(egui::Vec2 { x: 1366., y: 768. }),
        ..Default::default()
    };
    eframe::run_native(
        "Raytracer",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|_cc| Box::<MyApp>::default()),
            )
            .await
            .expect("failed to start eframe");
    });
}

struct MyApp {
    render: Option<TextureHandle>,
    width: usize,
    height: usize,
    camera: (Vector3<f32>, Rotation3<f32>),
    pub camera_transform: CameraTransform,
    pub scene: Vec<Sphere>,
    pub light: Vector3<f32>,
    pub ambiant: f32,
    pixels: Vec<Color32>,
    time: f32,
}

impl MyApp {
    fn scene_ui(&mut self, ui: &mut Ui, panel_width: f32) {
        let mut delete = None;
        ui.add_space(10.);
        ui.heading("Scene menu");
        ui.add_space(10.);
        for (i, sphere) in &mut (self.scene.iter_mut().enumerate()) {
            ui.collapsing(format!("Sphere {}", i), |ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .min_col_width(panel_width / 3.)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("x position");
                        ui.add(egui::DragValue::new(&mut sphere.center.x).speed(0.1));
                        ui.end_row();
                        ui.label("y position");
                        ui.add(egui::DragValue::new(&mut sphere.center.y).speed(0.1));
                        ui.end_row();
                        ui.label("z position");
                        ui.add(egui::DragValue::new(&mut sphere.center.z).speed(0.1));
                        ui.end_row();
                        ui.label("ray");
                        ui.add(egui::DragValue::new(&mut sphere.ray).speed(0.1));
                        ui.end_row();
                        ui.label("color");
                        let mut color = [sphere.color.x, sphere.color.y, sphere.color.z];
                        ui.color_edit_button_rgb(&mut color);
                        sphere.color = Vector3::from_column_slice(&color);
                        ui.end_row();
                    });
                if ui.button("🗙").clicked() {
                    delete = Some(i);
                }
            });
        }
        if ui.button("➕").clicked() {
            self.scene.push(Sphere {
                center: Vector3::zeros(),
                ray: 1.,
                color: Vector3::new(1., 1., 1.),
            });
        }

        if let Some(i) = delete {
            self.scene.remove(i);
        }
    }

    fn light_dark_mode_switcher(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top(egui::Id::new("top panel")).show(ctx, |ui| {
            let icon = if ui.visuals().dark_mode {
                "☀"
            } else {
                "🌙"
            };
            if ui.button(icon).clicked() {
                let visuals = if ui.visuals().dark_mode {
                    Visuals::light()
                } else {
                    Visuals::dark()
                };
                ctx.set_visuals(visuals);
            }
        });
    }

    fn render(&mut self, ui: &mut Ui) {
        self.camera = self.camera_transform.update();
        #[cfg(not(target_arch = "wasm32"))]
        let now = Instant::now();
        let pixels = self
            .pixels
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let viewport_pos = get_vector_from_index(i, self.width, self.height, &self.camera);
                let col = Color32::BLACK;
                let ray = Ray {
                    position: self.camera.0,
                    direction: viewport_pos - self.camera.0,
                };
                match ray.cast(&self.scene, &self.light, self.ambiant, 2) {
                    Some(c) => c.as_color(),
                    None => col,
                }
            })
            .collect::<Vec<_>>();
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.time = now.elapsed().as_secs_f32();
        }
        self.render = Some(ui.ctx().load_texture(
            "render",
            ColorImage {
                pixels,
                size: [self.width, self.height],
            },
            Default::default(),
        ));
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let pixels = (0..(600 * 800)).map(|_| Color32::WHITE).collect::<Vec<_>>();
        let scene = vec![
            Sphere {
                color: Vector3::new(0.75, 0.66, 0.45),
                ray: 1.,
                center: Vector3::new(0., 0., 3.),
            },
            Sphere {
                color: Vector3::new(0.0, 0.45, 0.99),
                ray: 85.,
                center: Vector3::new(0., -86.5, 3.),
            },
        ];
        Self {
            render: Default::default(),
            width: 800,
            height: 600,
            pixels,
            time: 0.0,
            camera: (Vector3::new(0., 0., -5.), Rotation3::identity()),
            camera_transform: Default::default(),
            light: Vector3::zeros(),
            ambiant: 0.15,
            scene,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        let panel_width = frame.info().window_info.size.x / 6.;
        #[cfg(target_arch = "wasm32")]
        let panel_width = (web_sys::window()
            .unwrap()
            .inner_width()
            .unwrap()
            .as_f64()
            .unwrap()
            / 6.) as f32;
        self.light_dark_mode_switcher(ctx);
        egui::SidePanel::left(egui::Id::new("left panel"))
            .min_width(panel_width)
            .show(ctx, |ui| {
                self.scene_ui(ui, panel_width);
            });
        egui::SidePanel::right(egui::Id::new("right panel"))
            .min_width(panel_width)
            .show(ctx, |ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .min_col_width(panel_width / 3.)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.heading("Informations");
                        ui.end_row();
                        ui.label("Time to render in seconds");
                        ui.label(format!("{}", self.time));
                        ui.end_row();
                        ui.label("frames per second (fps)");
                        ui.label(format!("{}", 1. / self.time));
                        ui.end_row();

                        ui.heading("directional Light Menu");
                        ui.end_row();
                        ui.label("Light x");
                        ui.add(egui::DragValue::new(&mut self.light.x).speed(0.01));
                        ui.end_row();
                        ui.label("Light y");
                        ui.add(egui::DragValue::new(&mut self.light.y).speed(0.01));
                        ui.end_row();
                        ui.label("Light z");
                        ui.add(egui::DragValue::new(&mut self.light.z).speed(0.01));
                        ui.end_row();

                        ui.heading("Camera menu");
                        ui.end_row();
                        ui.colored_label(Color32::GREEN, "Camera rotation");
                        ui.end_row();
                        ui.label("Camera x rotation");
                        ui.add(egui::DragValue::new(&mut self.camera_transform.rot_x).speed(0.05));
                        ui.end_row();
                        ui.label("Camera y rotation");
                        ui.add(egui::DragValue::new(&mut self.camera_transform.rot_y).speed(0.05));
                        ui.end_row();
                        ui.label("Camera z rotation");
                        ui.add(egui::DragValue::new(&mut self.camera_transform.rot_z).speed(0.05));
                        ui.end_row();

                        ui.colored_label(Color32::GREEN, "Camera position");
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
