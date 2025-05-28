//! Text

use core::fmt;

use ab_glyph::{FontRef, InvalidFont, PxScale};
use imageproc::drawing::draw_text_mut;

use crate::{Color, Image};

/// Description of the generated OG Image
pub struct Text<'font> {
    /// Contents of the text
    pub content: String,
    /// Color of the text
    pub color: Color,
    /// Size of the text
    pub size: u32,
    /// x-coordinate of the top left corner of the text
    pub x: i32,
    /// y-coordinate of the top left corner of the text
    pub y: i32,
    /// Bytes of the font
    ///
    /// Example:
    ///
    /// ```ignore
    /// include_bytes!("../JetBrainsMono.ttf")
    /// ```
    pub font: &'font [u8],
}

impl fmt::Debug for Text<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Text")
            .field("color", &self.color)
            .field("size", &self.size)
            .field("content", &self.content)
            .finish()
    }
}

impl Text<'_> {
    /// Draw the text on the image
    pub fn draw(self, img: &mut Image) -> Result<(), InvalidFont> {
        draw_text_mut(
            img,
            self.color.into(),
            self.x,
            self.y,
            PxScale {
                x: self.size as f32,
                y: self.size as f32,
            },
            &FontRef::try_from_slice(self.font)?,
            &self.content,
        );

        Ok(())
    }
}
