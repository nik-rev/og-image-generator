//! OG Image generation

mod background;
mod color;
mod og_image;
mod text;

pub use background::Background;
pub use color::Color;
pub use text::Text;

type Image = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
