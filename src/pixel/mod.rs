/// Monochromatic pixels. Effectively represent RGB pixels, except each channel is equivalent.
pub mod mono;

/// RGB pixels. Have 3 components for Red, Green, and Blue.
pub mod rgb;

/// HSL pixels. Have 3 components for Hue, Saturation, and Luminance.
pub mod hsl;

/// LAB pixels. Have 3 components for Luma, a, and b.
pub mod lab;

/// LCH pixels. Have 3 components for Luma, Chroma, and Hue.
pub mod lch;

pub mod oklab;

pub mod oklch;