//! Render OG image to HTML

mod html_to_image;
use std::{fs, path::PathBuf};

use clap::Parser as _;

#[derive(clap::Parser)]
struct Args {
    /// Path of the HTML file to render
    input: PathBuf,
    /// Path of the output image file. Can be any extension such as .png, .jpg, .webp or others
    output: PathBuf,
    /// Width of the produced image
    #[arg(long, default_value_t = 1280)]
    width: u32,
    /// Height of the produced image
    #[arg(long, default_value_t = 675)]
    height: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let output = html_to_image::html_to_image(
        args.width,
        args.height,
        &fs::read_to_string(args.input).unwrap(),
    )
    .await
    .unwrap();

    output.save(args.output).unwrap();
}
