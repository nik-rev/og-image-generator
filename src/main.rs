//! Render OG image to HTML

mod cli;
mod html_to_image;
use std::{fs, path::Path};

use anyhow::anyhow;
use clap::Parser as _;
use cli::Cli;
use scraper::Selector;
use tap::Pipe as _;
use walkdir::WalkDir;

/// Extract all `key` and `value` fields from `<meta>` tags in a HTML file of the form:
///
/// ```html
/// <meta property="og-image-generator" key="key" value="value" />
/// ```
fn scrape_tera_vars(path: &Path) -> anyhow::Result<Vec<(String, String)>> {
    let s = fs::read_to_string(path)?;

    let html = scraper::Html::parse_document(&s);
    let selector =
        Selector::parse(r#"meta[property="og-image-generator"]"#).expect("valid selector");

    html.select(&selector)
        .filter_map(|node| {
            node.attr("key").and_then(|key| {
                node.attr("value")
                    .map(|value| (key.to_string(), value.to_string()))
            })
        })
        .collect::<Vec<_>>()
        .pipe(Ok)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Cli::parse();

    match args.command {
        cli::Command::Single {
            template,
            output,
            width,
            height,
            font,
            scrape,
            variables,
        } => {
            let mut tera_vars = scrape
                .as_deref()
                .map(scrape_tera_vars)
                .transpose()?
                .unwrap_or(vec![]);

            tera_vars.extend(variables);

            let og_image =
                html_to_image::html_to_image(width, height, &template, font.as_deref(), &tera_vars)
                    .await?;

            og_image.save(output)?;
        }
        cli::Command::All {
            template,
            width,
            image_file,
            height,
            path,
            font,
        } => {
            for (og_image_path, html_file) in WalkDir::new(path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|file| file.path().extension().is_some_and(|ext| ext == "html"))
                .filter_map(|html_file| {
                    if html_file
                        .path()
                        .file_name()
                        .is_some_and(|name| name == "index.html")
                    {
                        // foo/index.html
                        //
                        // We want to create og.png in the same directory
                        //
                        // foo/og.png
                        let path = html_file.into_path();
                        let Some(parent) = path.parent() else {
                            log::warn!("Failed to get parent of {}", path.display());
                            return None;
                        };

                        Some((parent.join(&image_file), path))
                    } else {
                        // foo.html
                        //
                        // We want to create directory foo with contents "og.png"
                        let html_file = html_file.into_path();
                        let Some(og_image_parent_dir) = html_file
                            .to_str()
                            .and_then(|file| file.strip_suffix(".html"))
                            .map(Path::new)
                        else {
                            log::warn!("Failed to get image directory of {}", html_file.display());
                            return None;
                        };
                        if let Err(err) = fs::create_dir_all(og_image_parent_dir) {
                            log::error!(
                                "Failed to create directory {}: {err}",
                                og_image_parent_dir.display()
                            );
                        }

                        Some((og_image_parent_dir.join(&image_file), html_file))
                    }
                })
            {
                let tera_vars = html_file.pipe_deref(scrape_tera_vars).unwrap_or(vec![]);

                let og_image = html_to_image::html_to_image(
                    width,
                    height,
                    &template,
                    font.as_deref(),
                    &tera_vars,
                )
                .await?;

                og_image
                    .save(og_image_path)
                    .map_err(|err| anyhow!("Failed to save the image: {err}"))?;
            }
        }
    }

    Ok(())
}
