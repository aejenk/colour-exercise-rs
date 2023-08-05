use super::{lab::LabPixel, rgb::RgbPixel};
use crate::{conversions::{lab_to_lch, lch_to_lab}, comparisons::{ciede2000, cie94}};

#[derive(Debug, Clone, Copy)]
/// The 3 components of an LCH pixel are as follows:
/// 
/// - Lightness: Ranges from 0.0 to 100.0. Determines the visible luminance of the pixel.
/// - Chroma: Ranges from 0.0 to 150.0. Effectively determines the *saturation* of the pixel.
/// - Hue: Ranges from 0.0 to 360.0.
pub struct LchPixel(pub f32, pub f32, pub f32);

pub mod colours {
    use super::LchPixel;

    // 1-bit
    pub static BLACK: LchPixel = LchPixel(0.0, 0.0, 0.0);
    pub static WHITE: LchPixel = LchPixel(100.0, 0.0, 0.0);
}

impl From<(f32, f32, f32)> for LchPixel {
    fn from(value: (f32, f32, f32)) -> Self {
        let (l, c, h) = value;
        LchPixel(l, c, h)
    }
}

impl From<RgbPixel> for LchPixel {
    fn from(value: RgbPixel) -> Self {
        Self::from_lab(&LabPixel::from_rgb(&value))
    }
}

impl From<LabPixel> for LchPixel {
    fn from(value: LabPixel) -> Self {
        Self::from_lab(&value)
    }
}

impl Into<RgbPixel> for LchPixel {
    fn into(self) -> RgbPixel {
        self.as_lab().as_rgb()
    }
}

impl Into<LabPixel> for LchPixel {
    fn into(self) -> LabPixel {
        self.as_lab()
    }
}

impl LchPixel {
    pub fn get(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }

    pub fn add_luma(&mut self, luma: f32) -> &mut Self {
        self.0 = (self.0 + luma).clamp(0.0, 100.0);
        self
    }

    pub fn add_chroma(&mut self, chroma: f32) -> &mut Self {
        self.1 = (self.1 + chroma).clamp(0.0, 132.0);
        self
    }

    pub fn add_hue(&mut self, hue: f32) -> &mut Self {
        self.2 = self.2 + hue;
        self
    }

    pub fn quantize_hue(&mut self, hues: &[f32]) -> &mut Self {
        let mut closest_dist = f32::MAX;
        let pixel_hue = ((self.2 % 360.0) + 360.0) % 360.0;
        let mut current_hue = pixel_hue;

        for hue in hues.iter() {
            let normalized = ((hue % 360.0) + 360.0) % 360.0;
            let distance = (normalized - pixel_hue).abs();
            if distance < closest_dist {
                closest_dist = distance;
                current_hue = normalized;
            }
        }

        self.2 = current_hue;
        self
    }

    /// Utilizes CIE94 to allow calculating colour differences with LCH
    pub fn distance_from(&self, other: &LchPixel) -> f32 {
        cie94(self.get(), other.get())
    }

    pub fn quantize(&self, palette: &[LchPixel]) -> LchPixel {
        let mut closest_distance = f32::MAX;
        let mut current_colour = self;

        for colour in palette.iter() {
            let distance = colour.distance_from(self);
            if distance < closest_distance {
                current_colour = colour;
                closest_distance = distance;
            };
        }

        current_colour.get().into()
    }

    pub fn from_lab(lab: &LabPixel) -> LchPixel {
        lab_to_lch(lab.get()).into()
    }

    pub fn from_rgb(rgb: &RgbPixel) -> LchPixel {
        rgb.as_lab().as_lch()
    }

    pub fn as_lab(&self) -> LabPixel {
        lch_to_lab(self.get()).into()
    }

    pub fn as_rgb(&self) -> RgbPixel {
        self.as_lab().as_rgb()
    }
}