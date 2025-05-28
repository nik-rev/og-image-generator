//! Open Graph image generator

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use image::{ImageReader, Rgba, RgbaImage, imageops::crop};
use resvg::{tiny_skia, usvg};
use tap::Pipe as _;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 675;

/// Background of the Open Graph image
enum Background {
    /// Background is path to an image
    Image(PathBuf),
    /// Background is path to an svg
    Svg(PathBuf),
    /// The background should be filled with a solid color
    Fill(Rgba<u8>),
}

fn create(bg: Background) -> Result<(), Box<dyn Error>> {
    let mut img = match bg {
        Background::Fill(rgb) => RgbaImage::from_pixel(WIDTH, HEIGHT, rgb),
        Background::Image(path) => ImageReader::open(path)?.decode()?.into(),
        Background::Svg(path) => {
            let mut opt = usvg::Options {
                // Get file's absolute directory.
                resources_dir: fs::canonicalize(&path)
                    .ok()
                    .and_then(|p| p.parent().map(Path::to_path_buf)),
                ..usvg::Options::default()
            };
            opt.fontdb_mut().load_system_fonts();

            let tree = fs::read(&path)?.pipe(|svg_data| usvg::Tree::from_data(&svg_data, &opt))?;

            let pixmap_size = tree.size().to_int_size();
            let mut pixmap =
                tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
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
            .ok_or("failed to encode")?
        }
    };
    crop(&mut img, 0, 0, WIDTH, HEIGHT);

    img.save(Path::new("output.png"))?;

    Ok(())
}

fn main() {
    create(Background::Svg(
        Path::new("default-template.svg").to_path_buf(),
    ))
    .unwrap();
}
