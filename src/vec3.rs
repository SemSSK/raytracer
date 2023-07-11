use egui::Color32;
use nalgebra::Vector3;

pub trait ConvertableToColor {
    fn as_color(&self) -> Color32;
}

impl ConvertableToColor for Vector3<f32> {
    fn as_color(&self) -> Color32 {
        let r = self.x * 255.;
        let g = self.y * 255.;
        let b = self.z * 255.;
        Color32::from_rgb(r as u8, g as u8, b as u8)
    }
}
