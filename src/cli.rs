use std::path::PathBuf;

use anstyle::{AnsiColor, Effects};

/// Styles for the CLI
const STYLES: clap::builder::Styles = clap::builder::Styles::styled()
    .header(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::BrightCyan.on_default())
    .error(AnsiColor::BrightRed.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .invalid(AnsiColor::BrightYellow.on_default().effects(Effects::BOLD));

/// Generate Open Graph images from HTML and CSS
#[derive(clap::Parser)]
#[command(arg_required_else_help = true, version, styles = STYLES, long_about = None)]
pub struct Args {
    /// Path of the HTML file to render
    pub input: PathBuf,
    /// Path of the output image file
    ///
    /// Can be any extension such as `.png`, `.jpg`, `.webp` or others
    #[arg(default_value = "output.png")]
    pub output: PathBuf,
    /// Width of the produced image
    #[arg(long, default_value_t = 1280)]
    pub width: u32,
    /// Height of the produced image
    #[arg(long, default_value_t = 675)]
    pub height: u32,
    /// Use a custom font
    ///
    /// The font can be referenced in HTML via `{{ font }}`
    ///
    /// Only `.ttf` extension is supported
    #[arg(long)]
    pub font: Option<PathBuf>,
    /// Variables of format `key=value` that will be available to use in the html
    ///
    /// Each `key` can be used in the HTML as `{{ key }}` and will expand to `value`
    ///
    /// `{{ height }}` and `{{ width }}` is always available
    #[arg(value_parser = |s: &str|
        s.split_once('=')
        .ok_or("expected `key=value`")
        .map(|(s1, s2)| (s1.to_string(), s2.to_string())
    ))]
    pub variables: Vec<(String, String)>,
}
