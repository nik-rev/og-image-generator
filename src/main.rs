//! OG image generator

use std::path::Path;

use og_image_generator::{Background, Color, OgImage, Text};

fn create(bg: Background) -> Result<(), Box<dyn std::error::Error>> {
    let font = include_bytes!("../Literata-VariableFont.ttf");

    let og = OgImage::builder()
        .title(Text {
            content: "This is a blog post".to_string(),
            color: Color::gray(10),
            size: 60,
            font,
        })
        .background(bg)
        .description(Text {
            content: "This is a description".to_string(),
            color: Color::gray(40),
            size: 40,
            font,
        })
        .build()
        .create();

    og?.save(Path::new("output.png"))?;

    Ok(())
}

fn main() {
    create(Background::Svg(
        Path::new("default-template.svg").to_path_buf(),
    ))
    .unwrap();
}
