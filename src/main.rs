//! Render OG image to HTML

mod html_to_image;
use std::path::PathBuf;

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
    /// Use a custom font. The font's can be referenced in HTML by its filename, excluding the extension
    /// For example, `Monaspace-Regular.ttf` will be available as `"Monaspace-Regular"`
    ///
    /// Only `.ttf` extension is supported.
    #[arg(long, verbatim_doc_comment)]
    font: Option<PathBuf>,
    /// Variables of format `key=value` that will be available to use in the html.
    ///
    /// Each `key` can be used in the HTML as `{{ key }}` and will expand to `value`.
    ///
    /// `{{ height }}` and `{{ width }}` is always available.
    #[arg(value_parser = |s: &str|
        s.split_once('=')
        .ok_or("expected `key=value`")
        .map(|(s1, s2)| (s1.to_string(), s2.to_string())
    ))]
    variables: Vec<(String, String)>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let output = html_to_image::html_to_image(
        args.width,
        args.height,
        &args.input,
        args.font.as_deref(),
        &args.variables,
    )
    .await
    .unwrap();

    output.save(args.output).unwrap();
}
