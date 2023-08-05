use crate::{comparisons::cie94, conversions::{oklab_to_oklch, oklch_to_oklab}};

use super::{oklab::OklabPixel, rgb::RgbPixel};

#[derive(Debug, Clone, Copy)]
/// The 3 components of an OKLCH pixel are as follows:
/// 
/// - Lightness: Ranges from 0.0 to 1.0. Determines the visible luminance of the pixel.
/// - Chroma: Ranges from 0.0 to 0.4. Effectively determines the *saturation* of the pixel.
/// - Hue: Ranges from 0.0 to 360.0.
pub struct OklchPixel(pub f32, pub f32, pub f32);

impl From<(f32, f32, f32)> for OklchPixel {
    fn from(value: (f32, f32, f32)) -> Self {
        let (l, c, h) = value;
        OklchPixel(l, c, h)
    }
}

impl OklchPixel {
    pub fn get(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }

    pub fn add_luma(&mut self, luma: f32) -> &mut Self {
        self.0 = (self.0 + luma).clamp(0.0, 1.0);
        self
    }

    pub fn add_chroma(&mut self, chroma: f32) -> &mut Self {
        self.1 = (self.1 + chroma).clamp(0.0, 0.4);
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

    pub fn distance_from(&self, other: &OklchPixel) -> f32 {
        cie94(self.get(), other.get())
    }

    pub fn quantize(&self, palette: &[OklchPixel]) -> OklchPixel {
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

    pub fn from_oklab(oklab: &OklabPixel) -> OklchPixel {
        oklab_to_oklch(oklab.get()).into()
    }

    pub fn from_rgb(rgb: &RgbPixel) -> OklchPixel {
        rgb.as_oklab().as_oklch()
    }

    pub fn as_oklab(&self) -> OklabPixel {
        oklch_to_oklab(self.get()).into()
    }

    pub fn as_rgb(&self) -> RgbPixel {
        self.as_oklab().as_rgb()
    }
}