//! Color

/// Color is made from 4 channels: Red, Green and Blue
#[derive(Debug, Clone, Copy)]
pub struct Color(image::Rgba<u8>);

impl From<image::Rgba<u8>> for Color {
    fn from(value: image::Rgba<u8>) -> Self {
        Self(value)
    }
}
impl From<Color> for image::Rgba<u8> {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl Color {
    /// Create an opaque color with all channels set to the same value
    #[must_use]
    pub fn gray(c: u8) -> Self {
        image::Rgba([c, c, c, 255]).into()
    }

    /// Create a color with red, green, blue and alpha values
    #[must_use]
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        image::Rgba([red, green, blue, alpha]).into()
    }

    /// Create an opaque color with red, green and blue
    #[must_use]
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        image::Rgba([red, green, blue, 255]).into()
    }

    /// Creates a `Color` from a hex string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use og_image::Color;
    ///
    /// let color1 = Color::from_hex("#c0ffee").unwrap();
    /// let color2 = Color::rgb(192, 255, 238);
    ///
    /// assert_eq!(color1, color2);
    /// ```
    #[must_use]
    pub fn from_hex(hex: &str) -> Option<Self> {
        if !(hex.starts_with('#') && hex.len() == 7) {
            return None;
        }
        match [1..=2, 3..=4, 5..=6].map(|i| hex.get(i).and_then(|c| u8::from_str_radix(c, 16).ok()))
        {
            [Some(r), Some(g), Some(b)] => Some(Self::rgb(r, g, b)),
            _ => None,
        }
    }
}
