use super::{rgb::RgbPixel, lch::LchPixel};
use crate::{conversions::{chain_conversions, rgb_to_xyz_d65, xyz_d65_to_xyz_d50, xyz_d50_to_lab, lab_to_xyz_d50, xyz_d50_to_xyz_d65, xyz_d65_to_rgb}, comparisons::cie76};

#[derive(Debug, Clone, Copy)]
/// The 3 components of an LAB pixel are:
/// 
/// - L: Ranges from 0.0 to 100.0. Determines the visible luminance of the pixel.
/// - a: Ranges from -125.0 to 125.0. Represents the greenness to redness of the pixel.
/// - b: Ranges from -125.0 to 125.0. Represents the blueness to yellowness of the pixel.
/// 
/// The nature of this pixel can be a bit finnicky to play with. You may prefer to use
/// LCH - which replaces `a` and `b` with `Chroma` (saturation) and `Hue`.
pub struct LabPixel(pub f32, pub f32, pub f32);

impl From<(f32, f32, f32)> for LabPixel {
    fn from(value: (f32, f32, f32)) -> Self {
        let (l, a, b) = value;
        LabPixel(l, a, b)
    }
}

impl From<RgbPixel> for LabPixel {
    fn from(value: RgbPixel) -> Self {
        Self::from_rgb(&value)
    }
}

impl Into<RgbPixel> for LabPixel {
    fn into(self) -> RgbPixel {
        self.as_rgb()
    }
}

impl LabPixel {
    pub fn get(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }

    pub fn distance_from(&self, other: &LabPixel) -> f32 {
        cie76(self.get(), other.get())
    }

    pub fn quantize(&self, palette: &[LabPixel]) -> LabPixel {
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

    pub fn from_rgb(rgb: &RgbPixel) -> LabPixel {
        chain_conversions(rgb.get(), &[
            rgb_to_xyz_d65,
            xyz_d65_to_xyz_d50,
            xyz_d50_to_lab,
        ]).into()
    }

    pub fn as_rgb(&self) -> RgbPixel {
        chain_conversions(self.get(), &[
            lab_to_xyz_d50,
            xyz_d50_to_xyz_d65,
            xyz_d65_to_rgb,
        ]).into()
    }

    pub fn as_lch(&self) -> LchPixel {
        LchPixel::from_lab(self)
    }
}