//! OG image generator

use std::path::Path;

use og_image_generator::Background;

fn create(bg: &Background) -> Result<(), Box<dyn std::error::Error>> {
    let img = bg.image(1280, 675)?;

    img.save(Path::new("output.png"))?;

    Ok(())
}

fn main() {
    create(&Background::Svg(
        Path::new("default-template.svg").to_path_buf(),
    ))
    .unwrap();
}
