use std::path::PathBuf;

use anstyle::{AnsiColor, Effects};
use clap::Subcommand;

// Styles for the CLI
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
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Generate a single open graph image from a template and variables
    Single {
        /// Path of the HTML template to render
        #[arg(value_name = "FILE")]
        template: PathBuf,
        /// Path of the output image file
        ///
        /// Can be any extension such as .png, .jpg, .webp or others
        #[arg(default_value = "output.png", value_name = "FILE")]
        output: PathBuf,
        /// Width of the produced image (pixels)
        #[arg(long, default_value_t = 1280)]
        width: u32,
        /// Height of the produced image (pixels)
        #[arg(long, default_value_t = 675)]
        height: u32,
        /// Scrapes the given HTML file for variables
        ///
        /// Finds all meta tags with property="og-image-generator",
        /// taking their key and value fields and passing them to the template.
        ///
        /// For example, given a tag like <meta property="og-image-generator" key="description" value="value" />
        /// it will add a variable {{ description }} which expands to value in the HTML template.
        #[arg(long)]
        scrape: Option<PathBuf>,
        /// Use a custom font
        ///
        /// The font can be referenced in HTML via {{ font }}
        ///
        /// Only .ttf extension is supported
        #[arg(long)]
        font: Option<PathBuf>,
        /// Variables of format key=value that will be available to use in the html
        ///
        /// Each key can be used in the HTML as {{ key }} and will expand to value
        ///
        /// {{ height }} and {{ width }} is always available
        #[arg(value_parser = |s: &str|
        s.split_once('=')
        .ok_or("expected key=value")
        .map(|(s1, s2)| (s1.to_string(), s2.to_string())
    ))]
        variables: Vec<(String, String)>,
    },
    /// Generate open graph images for all .html files in a directory,
    /// recursively, using variables found in <meta property="og-image-generator" key="..." value="..." />
    /// passing to the given template.
    ///
    /// - For a path/to/file.html, an open graph image will be generated at path/to/file/og.png
    /// - For a path/to/index.html, an open graph image will be generated at path/to/og.png
    All {
        /// Path of the HTML template to render
        #[arg(value_name = "FILE")]
        template: PathBuf,
        /// Path to the directory, which will be recursively searched for .html files
        /// to generate an open graph image next to each one
        path: PathBuf,
        /// Width of the produced image (pixels)
        #[arg(long, default_value_t = 1280)]
        width: u32,
        /// Height of the produced image (pixels)
        #[arg(long, default_value_t = 675)]
        height: u32,
        /// Name of the generated image file.
        #[arg(long, default_value = "og.png")]
        image_file: String,
        /// Use a custom font
        ///
        /// The font can be referenced in HTML via {{ font }}
        ///
        /// Only .ttf extension is supported
        #[arg(long)]
        font: Option<PathBuf>,
    },
}
