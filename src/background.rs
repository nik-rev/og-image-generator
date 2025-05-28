//! Background of an OG image

use crate::Error;
use crate::Image;
use std::{
    fs,
    path::{Path, PathBuf},
};

use image::{ImageReader, Rgba, RgbaImage};
use resvg::{tiny_skia, usvg};
use tap::Pipe as _;

/// Background of the Open Graph image
#[derive(Clone, Debug)]
pub enum Background {
    /// Background is path to an image
    Image(PathBuf),
    /// Background is path to an svg
    Svg(PathBuf),
    /// The background should be filled with a solid color
    Fill(Rgba<u8>),
}

impl Default for Background {
    /// The default background is white
    fn default() -> Self {
        Self::Fill(Rgba([255, 255, 255, 255]))
    }
}

/// Read svg string from the path into an Rgba image
fn read_svg_to_image(path: &Path) -> Result<Image, Error> {
    let mut opt = usvg::Options {
        // Get file's absolute directory.
        resources_dir: fs::canonicalize(path)
            .ok()
            .and_then(|p| p.parent().map(Path::to_path_buf)),
        ..usvg::Options::default()
    };
    opt.fontdb_mut().load_system_fonts();

    let tree = fs::read(path)
        .map_err(Error::OpenBackgroundSvg)?
        .pipe(|svg| usvg::Tree::from_data(&svg, &opt))
        .map_err(Error::ParseBackgroundSvg)?;

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or(Error::CreatePixmapForBackground)?;
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    RgbaImage::from_raw(
        pixmap_size.width(),
        pixmap_size.height(),
        pixmap
            .pixels()
            .iter()
            .flat_map(|pixel| [pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
            .collect(),
    )
    .ok_or(Error::BackgroundSvgIntoImage)
}

impl Background {
    /// Get the image corresponding to this background
    pub fn image(&self, width: u32, height: u32) -> Result<Image, Error> {
        match self {
            Self::Fill(rgb) => RgbaImage::from_pixel(width, height, *rgb),
            Self::Image(path) => ImageReader::open(path)
                .map_err(Error::OpenBackgroundImage)?
                .decode()
                .map_err(Error::DecodeBackgroundImage)?
                .into(),
            Self::Svg(path) => read_svg_to_image(path)?,
        }
        .pipe(|mut image| {
            image::imageops::crop(&mut image, 0, 0, width, height);
            Ok(image)
        })
    }
}
