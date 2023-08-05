use crate::conversions::{chain_conversions, rgb_to_xyz_d65, xyz_d65_to_oklab, oklab_to_xyz_d65, xyz_d65_to_rgb};

use super::{rgb::RgbPixel, oklch::OklchPixel};

#[derive(Debug, Clone, Copy)]
/// The 3 components of an OKLAB pixel are:
/// 
/// - L: Ranges from 0.0 to 1.0. Determines the visible luminance of the pixel.
/// - a: Ranges from -0.4 to 0.4. Represents the greenness to redness of the pixel.
/// - b: Ranges from -0.4 to 0.4. Represents the blueness to yellowness of the pixel.
/// 
/// The nature of this pixel can be a bit finnicky to play with. You may prefer to use
/// OKLCH - which replaces `a` and `b` with `Chroma` (saturation) and `Hue`.
pub struct OklabPixel(pub f32, pub f32, pub f32);

impl From<(f32, f32, f32)> for OklabPixel {
    fn from(value: (f32, f32, f32)) -> Self {
        let (l, a, b) = value;
        OklabPixel(l, a, b)
    }
}

impl OklabPixel {
    pub fn get(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }

    pub fn from_rgb(rgb: &RgbPixel) -> OklabPixel {
        chain_conversions(rgb.get(), &[
            rgb_to_xyz_d65,
            xyz_d65_to_oklab,
        ]).into()
    }

    pub fn as_rgb(&self) -> RgbPixel {
        chain_conversions(self.get(), &[
            oklab_to_xyz_d65,
            xyz_d65_to_rgb,
        ]).into()
    }

    pub fn as_oklch(&self) -> OklchPixel {
        OklchPixel::from_oklab(self)
    }
}