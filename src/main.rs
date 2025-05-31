//! Render OG image to HTML

mod cli;
mod html_to_image;
use clap::Parser as _;
use cli::Args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let output = html_to_image::html_to_image(
        args.width,
        args.height,
        &args.input,
        args.font.as_deref(),
        &args.variables,
    )
    .await?;

    output.save(args.output)?;

    Ok(())
}
