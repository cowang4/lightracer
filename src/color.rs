
use std::convert::Into;
use std::ops::{Add, AddAssign, Div, Mul};

use image::{Rgba, Pixel};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            red: self.red+other.red,
            green: self.green+other.green,
            blue: self.blue+other.blue,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = Color {
            red: self.red+other.red,
            green: self.green+other.green,
            blue: self.blue+other.blue,
        };
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red*other,
            green: self.green*other,
            blue: self.blue*other,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red*other.red,
            green: self.green*other.green,
            blue: self.blue*other.blue,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;
    fn div(self, other: f32) -> Color {
        Color {
            red: self.red/other,
            green: self.green/other,
            blue: self.blue/other,
        }
    }
}

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

impl Color {
    pub fn black() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn sky() -> Color {
        Color {
            red: 0.01,
            green: 0.05,
            blue: 0.1,
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }
}

impl Into<Rgba<u8>> for Color {
    fn into(self) -> Rgba<u8> {
        self.clamp();
        Rgba::from_channels((gamma_encode(self.red) * 255.0) as u8,
                            (gamma_encode(self.green) * 255.0) as u8,
                            (gamma_encode(self.blue) * 255.0) as u8,
                            255)
    }
}

impl From<Rgba<u8>> for Color {
    fn from(rgba: Rgba<u8>) -> Color {
        Color {
            red: gamma_decode((rgba.data[0] as f32) / 255.0),
            green: gamma_decode((rgba.data[1] as f32) / 255.0),
            blue: gamma_decode((rgba.data[2] as f32) / 255.0),
        }
    }
}
