use super::{rgb::RgbPixel};
use crate::conversions::{rgb_to_hsl, hsl_to_rgb};

#[derive(Debug, Clone, Copy)]
/// Represents a pixel in the HSL colour space. Saturation and luminance are clamped at `0.0` to `1.0` - whereas hue can be any valid `f32` value.
/// The 3 components of an HSL pixel are as follows:
/// 
/// - Hue: Ranges from 0.0 to 360.0.
/// - Saturation: Ranges from 0.0 to 1.0. (May not be currently true.)
/// - Lightness: Ranges from 0.0 to 1.0 (May not currently be true.)
/// 
/// This is an improvement over RGB, however you may want to use LCH instead whose
/// components more accurately reflect human vision.
pub struct HslPixel(pub f32, pub f32, pub f32);

impl From<(f32, f32, f32)> for HslPixel {
    fn from(value: (f32, f32, f32)) -> Self {
        HslPixel(value.0, value.1, value.2)
    }
}

impl From<RgbPixel> for HslPixel {
    fn from(value: RgbPixel) -> Self {
        Self::from_rgb(&value)
    }
}

impl Into<RgbPixel> for HslPixel {
    fn into(self) -> RgbPixel {
        self.as_rgb()
    }
}

impl HslPixel {
    /// Adds (rotates) the hue.
    pub fn add_hue(&mut self, hue: f32) -> &mut Self {
        self.0 = self.0 + hue;
        self
    }

    /// Adds saturation. Any value can be passed, but the value on the pixel is clamped to `0.0` to `1.0`.
    pub fn add_saturation(&mut self, saturation: f32) -> &mut Self {
        self.1 = (self.1 + saturation).clamp(0.0, 1.0);
        self
    }

    /// Adds luminance. Any value can be passed, but the value on the pixel is clamped to `0.0` to `1.0`.
    pub fn add_luminance(&mut self, luminance: f32) -> &mut Self {
        self.2 = (self.2 + luminance).clamp(0.0, 1.0);
        self
    }

    pub fn quantize_hue(&mut self, hues: &[f32]) -> &mut Self {
        let mut closest_dist = f32::MAX;
        let pixel_hue = self.get_normalized_hue();
        let mut current_hue = pixel_hue;

        for hue in hues.iter() {
            let normalized = Self::normalize_hue(*hue);
            let distance = (normalized - pixel_hue).abs();
            if distance < closest_dist {
                closest_dist = distance;
                current_hue = normalized;
            }
        }

        self.0 = current_hue;
        self
    }

    /// Retrieves the hue as a value between 0 and 360.
    fn get_normalized_hue(&self) -> f32 {
        Self::normalize_hue(self.0)
    }

    fn normalize_hue(hue: f32) -> f32 {
        loop {
            if hue >= 0.0 {
                break hue % 360.0;
            } else {
                break (hue % 360.0) + 360.0;
            }
        }
    }

    /// Retrieves the (h, s, l) values.
    pub fn get(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }

    pub fn from_rgb(rgb: &RgbPixel) -> HslPixel {
        rgb_to_hsl(rgb.get()).into()
    }

    pub fn as_rgb(&self) -> RgbPixel {
        hsl_to_rgb(self.get()).into()
    }
}
