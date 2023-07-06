use std::ops::{Add, Mul, Sub};

use egui::Color32;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Vec3 {
    pub fn scale(&self, t: f32) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.x,
            z: self.x * rhs.y - self.y * rhs.z,
        }
    }
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn normalized(&self) -> Self {
        self.scale(1. / self.length())
    }
}

pub trait ConvertableToColor {
    fn as_color(&self) -> Color32;
}

impl ConvertableToColor for Vec3 {
    fn as_color(&self) -> Color32 {
        Color32::from_rgb(
            (self.x.abs() * 255.) as u8,
            (self.y.abs() * 255.) as u8,
            (self.z.abs() * 255.) as u8,
        )
    }
}
