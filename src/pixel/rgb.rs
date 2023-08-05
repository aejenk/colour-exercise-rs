use crate::comparisons::rgb_weighted_euclidean;

use super::{hsl::HslPixel, lab::LabPixel, lch::LchPixel, oklab::OklabPixel, oklch::OklchPixel};

#[derive(Debug, Clone, Copy)]
/// Represents a pixel in the RGB colour space. Each value (RGB) ranges between 0.0 and 1.0.
pub struct RgbPixel(pub f32, pub f32, pub f32);

pub mod colours {
    use super::RgbPixel;

    // 1-bit
    pub static BLACK: RgbPixel = RgbPixel(0.0, 0.0, 0.0);
    pub static WHITE: RgbPixel = RgbPixel(1.0, 1.0, 1.0);

    // primary colours
    pub static RED: RgbPixel = RgbPixel(1.0, 0.0, 0.0);
    pub static GREEN: RgbPixel = RgbPixel(0.0, 1.0, 0.0);
    pub static BLUE: RgbPixel = RgbPixel(0.0, 0.0, 1.0);

    // secondary colours
    pub static YELLOW: RgbPixel = RgbPixel(1.0, 1.0, 0.0);
    pub static PURPLE: RgbPixel = RgbPixel(1.0, 0.0, 1.0);
    pub static CYAN: RgbPixel = RgbPixel(0.0, 1.0, 1.0);

    // other
    pub static PINK: RgbPixel = RgbPixel(1.0, 0.6, 0.8);
    pub static MAGENTA: RgbPixel = RgbPixel(1.0, 0.15, 0.8);
    pub static ROSE: RgbPixel = RgbPixel(1.0, 0.0, 0.59);

    pub static GOLD: RgbPixel = RgbPixel(1.0, 0.8, 0.16);
    pub static ORANGE: RgbPixel = RgbPixel(1.0, 0.4, 0.0);
    pub static RUST: RgbPixel = RgbPixel(0.7, 0.2, 0.0);

    pub static AQUAMARINE: RgbPixel = RgbPixel(0.0, 1.0, 0.6);
}

impl From<(u8, u8, u8)> for RgbPixel {
    fn from(value: (u8, u8, u8)) -> Self {
        RgbPixel(
            value.0 as f32 / 255.0,
            value.1 as f32 / 255.0,
            value.2 as f32 / 255.0,
        )
    }
}

impl From<(f32, f32, f32)> for RgbPixel {
    fn from(value: (f32, f32, f32)) -> Self {
        RgbPixel(value.0, value.1, value.2)
    }
}

impl From<&str> for RgbPixel {
    fn from(value: &str) -> Self {
        let r = u8::from_str_radix(&value[0..=1], 16);
        let g = u8::from_str_radix(&value[2..=3], 16);
        let b = u8::from_str_radix(&value[4..=5], 16);

        if let (Ok(ru), Ok(gu), Ok(bu)) = (r, g, b) {
            RgbPixel(
                ru as f32 / 255.0,
                gu as f32 / 255.0,
                bu as f32 / 255.0,
            )
        } else {
            println!(
                "WARNING! Couldn't convert {} into an RGB value. Returning black.",
                value
            );
            RgbPixel(0.0, 0.0, 0.0)
        }
    }
}

impl RgbPixel {
    /// Retrieves the (r, g, b) channels of the pixel as a tuple.
    pub fn get(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }

    pub fn get_u8(&self) -> (u8, u8, u8) {
        (
            (self.0 * 255.0).round() as u8,
            (self.1 * 255.0).round() as u8,
            (self.2 * 255.0).round() as u8,
        )
    }

    pub fn clamp(&self) -> RgbPixel {
        (
            self.0.clamp(0.0, 1.0),
            self.1.clamp(0.0, 1.0),
            self.2.clamp(0.0, 1.0),
        ).into()
    }

    /// Adds an error to each of the channels.
    pub fn add_error(self, error: (f32, f32, f32)) -> RgbPixel {
        RgbPixel(
            (self.0 + error.0).min(1.0).max(0.0),
            (self.1 + error.1).min(1.0).max(0.0),
            (self.2 + error.2).min(1.0).max(0.0),
        )
    }

    /// Quantizes the RGB pixel to the nearest colour in the palette.
    pub fn quantize(&self, palette: &[RgbPixel]) -> RgbPixel {
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

    /// Mixes two colours together to produce a third colour.
    ///
    /// Takes a factor that determines how much priority to give the *current* pixel.
    ///
    /// - `0.5` mixes equally.
    /// - `> 0.5` prioritizes the *current* pixel.
    /// - `< 0.5` prioritizes the *other/parameter* pixel.
    ///
    /// Putting it another way:
    ///
    /// ```ignore
    /// RED.mix(&BLUE, 0.0) = BLUE
    /// RED.mix(&BLUE, 1.0) = RED
    /// ```
    pub fn mix(&self, ratio: f32, other: &RgbPixel) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        let mix_calc = |pixchan1: f32, pixchan2: f32| {
            (pixchan1 * ratio) + pixchan2 * (1.0 - ratio)
        };
        RgbPixel(
            mix_calc(self.0, other.0),
            mix_calc(self.1, other.1),
            mix_calc(self.2, other.2),
        )
    }

    /// This function will generate a list of colours with the same hue but
    /// varying brightness - by using the HSL colour space.
    ///
    /// `shades` determines how many shades get generated. Passing `1` will
    /// return a vector with a single colour containing `0.5` luminance - for example.
    ///
    /// **Note:** This will *not* include black and white.
    pub fn build_gradient_using_hsl(&self, shades: u16) -> Vec<Self> {
        let fractional = 1.0 / (shades + 1) as f32;
        (1..=shades)
            .into_iter()
            .map(|i| {
                self.as_hsl()
                    // set the luminance to black first
                    .add_luminance(-2.0)
                    .add_luminance(i as f32 * fractional)
                    .as_rgb()
                    .clamp()
            })
            .collect()
    }

    /// This function will generate a list of colours with the same hue but
    /// varying brightness - by using the OKLCH colour space.
    ///
    /// `shades` determines how many shades get generated. Passing `1` will
    /// return a vector with a single colour containing `0.5` luminance - for example.
    ///
    /// **Note:** This will *not* include black and white.
    pub fn build_gradient_using_oklch(&self, shades: u16) -> Vec<Self> {
        let fractional = 1.0 / (shades + 1) as f32;
        (1..=shades)
            .into_iter()
            .map(|i| {
                self.as_oklch()
                    // set the luminance to black first
                    .add_luma(-2.0)
                    .add_luma(i as f32 * fractional)
                    .as_rgb()
                    .clamp()
            })
            .collect()
    }

    /// This function will build a gradient by mixing the current colour with another
    /// using various ratios.
    ///
    /// `mixes` determines how many mixes get generated. Passing `1` will return
    /// a vector with a single colour that mixes both equally.
    ///
    /// **Note:** This will *not* include either **pure** colour - only mixes.
    pub fn build_gradient_mix(&self, other: &RgbPixel, mixes: u16) -> Vec<Self> {
        let fractional = 1 as f32 / (mixes + 1) as f32;
        (1..=mixes)
            .into_iter()
            .map(|i| self.mix(i as f32 * fractional, other))
            .collect()
    }

    /// Gets the error in channel values between itself and another `RgbPixel`.
    pub fn get_error(&self, other: &RgbPixel) -> (f32, f32, f32) {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
        )
    }

    /// Retrieves the distance between it and another `RgbPixel` using the
    /// weighted euclidean method.
    pub fn distance_from(&self, other: &RgbPixel) -> f32 {
        rgb_weighted_euclidean(self.get(), other.get())
    }

    /// Converts the pixel to an `HslPixel`.
    pub fn as_hsl(&self) -> HslPixel {
        HslPixel::from_rgb(self)
    }

    /// Converts the pixel to a `LabPixel`.
    pub fn as_lab(&self) -> LabPixel {
        LabPixel::from_rgb(self)
    }

    /// Converts the pixel to a `LchPixel`.
    pub fn as_lch(&self) -> LchPixel {
        LchPixel::from_rgb(self)
    }

    pub fn as_oklab(&self) -> OklabPixel {
        OklabPixel::from_rgb(self)
    }

    pub fn as_oklch(&self) -> OklchPixel {
        OklchPixel::from_rgb(self)
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::RgbPixel;

    const ITERATIONS: usize = 10_000;
    const TIME_SUFFIX: &'static str = "µs";
    const RGB_PIXEL: RgbPixel = RgbPixel(0.0, 0.0, 0.0);

    #[test]
    fn benchmarks() {
        benchmark_hsl();
        benchmark_lab();
        benchmark_lch();
        benchmark_oklab();
        benchmark_oklch();
    }

    fn benchmark_hsl() {
        let now = Instant::now();

        for _ in 1..ITERATIONS {
            RGB_PIXEL.as_hsl();
        }

        println!("HSL  : {}{}", now.elapsed().as_micros(), TIME_SUFFIX);
    }

    fn benchmark_lab() {
        let now = Instant::now();

        for _ in 1..ITERATIONS {
            RGB_PIXEL.as_lab();
        }

        println!("LAB  : {}{}", now.elapsed().as_micros(), TIME_SUFFIX);
    }

    fn benchmark_lch() {
        let now = Instant::now();

        for _ in 1..ITERATIONS {
            RGB_PIXEL.as_lch();
        }

        println!("LCH  : {}{}", now.elapsed().as_micros(), TIME_SUFFIX);
    }

    fn benchmark_oklab() {
        let now = Instant::now();

        for _ in 1..ITERATIONS {
            RGB_PIXEL.as_oklab();
        }

        println!("OKLAB: {}{}", now.elapsed().as_micros(), TIME_SUFFIX);
    }

    fn benchmark_oklch() {
        let now = Instant::now();

        for _ in 1..ITERATIONS {
            RGB_PIXEL.as_oklch();
        }

        println!("OKLCH: {}{}", now.elapsed().as_micros(), TIME_SUFFIX);
    }
}