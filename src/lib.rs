//! OG Image generation

/// Error trying to create the OG Image
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occured in the layout
    #[error("Error computing layout: {0}")]
    Layout(#[from] taffy::TaffyError),
    /// Font for title is not valid
    #[error(transparent)]
    InvalidTitleFont(InvalidFont),
    /// Font for description is not valid
    #[error(transparent)]
    InvalidDescriptionFont(InvalidFont),
    /// Failed to open the background image file
    #[error(transparent)]
    OpenBackgroundImage(io::Error),
    /// Failed to open the background svg file
    #[error(transparent)]
    OpenBackgroundSvg(io::Error),
    /// Failed to decode image
    #[error(transparent)]
    DecodeBackgroundImage(image::ImageError),
    /// Failed to parse the SVG
    #[error(transparent)]
    ParseBackgroundSvg(usvg::Error),
    /// Conversion of Svg into Image failed
    #[error("Failed to convert SVG into an Image")]
    BackgroundSvgIntoImage,
    /// Creation of Pixmap failed
    #[error("Failed to create the Pixmap")]
    CreatePixmapForBackground,
}

mod background;
mod color;
mod og_image;
mod text;

use std::io;

use ab_glyph::InvalidFont;
pub use background::Background;
pub use color::Color;
pub use og_image::OgImage;
use resvg::usvg;
pub use text::Text;

type Image = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
