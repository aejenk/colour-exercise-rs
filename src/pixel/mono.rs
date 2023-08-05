#[derive(Debug, Clone, Copy)]
/// Represents a monochromatic pixel. It only has one value, which effectively represents the luminance.
pub struct MonoPixel(u8);

pub const ONE_BIT: &'static [MonoPixel] = &[MonoPixel(0), MonoPixel(255)];

impl From<u8> for MonoPixel {
    fn from(value: u8) -> Self {
        MonoPixel(value)
    }
}

impl MonoPixel {
    /// Adds an error to the luminance of the pixel.
    pub fn add_error(self, error: i32) -> MonoPixel {
        MonoPixel((self.0 as i32 + error).min(255).max(0) as u8)
    }

    /// Quantizes the pixel to the nearest `MonoPixel` in the palette.
    pub fn quantize(&self, palette: &[MonoPixel]) -> MonoPixel {
        let mut closest_dist = u16::MAX;
        let mut closest_col = self;

        for colour in palette.iter() {
            let distance = (colour.0 as i16 - self.0 as i16).abs() as u16;
            if distance < closest_dist {
                closest_col = colour;
                closest_dist = distance;
            }
        }

        // println!("{} ---- quantized to ---> {}", self.0, closest_col.get());

        closest_col.get().into()
    }

    /// Retrieves the error between it and another `MonoPixel`.
    pub fn get_error(&self, other: &MonoPixel) -> i32 {
        self.0 as i32 - other.0 as i32
    }

    /// Retrieves the luminance of the pixel.
    pub fn get(&self) -> u8 {
        self.0
    }
}
