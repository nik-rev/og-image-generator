//! OG image generator

mod background;

use std::path::Path;

use background::Background;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 675;

fn create(bg: &Background) -> Result<(), Box<dyn std::error::Error>> {
    let img = bg.image(WIDTH, HEIGHT)?;

    img.save(Path::new("output.png"))?;

    Ok(())
}

fn main() {
    create(&Background::Svg(
        Path::new("default-template.svg").to_path_buf(),
    ))
    .unwrap();
}
