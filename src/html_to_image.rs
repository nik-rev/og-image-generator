use anyhow::{Context as _, Result, anyhow};
use anyrender::render_to_buffer;
use anyrender_vello_cpu::VelloCpuImageRenderer;
use blitz_dom::net::Resource;
use blitz_html::HtmlDocument;
use blitz_net::{MpscCallback, Provider};
use blitz_paint::paint_scene;
use blitz_traits::{
    ColorScheme, Viewport, navigation::DummyNavigationProvider, net::SharedProvider,
};
use image::RgbaImage;
use std::{
    fs,
    path::{self, Path},
    sync::Arc,
};
use tera::{Context, Tera};

const CUSTOM_FONT_NAME: &str = "custom-font";

pub async fn html_to_image(
    width: u32,
    height: u32,
    html_path: &Path,
    font_path: Option<&Path>,
    variables: &[(String, String)],
) -> Result<RgbaImage> {
    let html = &fs::read_to_string(html_path)
        .map_err(|err| anyhow!("Failed to read {html_path:?}: {err}"))?;
    let (mut recv, callback) = MpscCallback::new();
    let callback = Arc::new(callback);
    let net = Arc::new(Provider::new(callback));

    let mut tera_cx = Context::new();

    tera_cx.insert("height", &height);
    tera_cx.insert("width", &width);
    tera_cx.insert("font", CUSTOM_FONT_NAME);
    for (key, value) in variables {
        tera_cx.insert(key, value);
    }
    let html = Tera::default().render_str(html, &tera_cx)?;

    let mut html_document = HtmlDocument::from_html(
        &format!(
            "<!doctype html>
<html>
    <head>
        <style>
            * {{
              margin: 0px;
              padding: 0px;
            }}
            body {{
                width: {width}px;
                height: {height}px;
                overflow: hidden;
            }}
        </style>
    </head>
    <body>
      {html}
    </body>
</html>"
        ),
        // This allows using relative paths in our HTML file to
        // import things from the file system
        Some(format!(
            "file:///{}",
            path::absolute(html_path)?
                .to_str()
                .context("Invalid path")?
        )),
        Vec::new(),
        Arc::clone(&net) as SharedProvider<Resource>,
        None,
        Arc::new(DummyNavigationProvider),
    );

    if let Some(font_path) = font_path {
        let font =
            fs::read(font_path).map_err(|err| anyhow!("Failed reading {font_path:?}: {err}"))?;

        html_document.font_ctx.collection.register_fonts(
            font.into(),
            Some(fontique::FontInfoOverride {
                family_name: Some(CUSTOM_FONT_NAME),
                width: None,
                style: None,
                weight: None,
                axes: None,
            }),
        );
    }

    html_document
        .as_mut()
        .set_viewport(Viewport::new(width, height, 1.0, ColorScheme::Light));

    while !net.is_empty() {
        let Some((_, res)) = recv.recv().await else {
            break;
        };
        html_document.as_mut().load_resource(res);
    }

    // Compute style & layout
    html_document.as_mut().resolve();

    let computed_height = html_document
        .as_ref()
        .root_element()
        .final_layout
        .size
        .height;

    let height = f64::from(computed_height)
        .max(f64::from(height))
        .min(4000.0) as u32;

    let buf = render_to_buffer::<VelloCpuImageRenderer, _>(
        |scene| {
            paint_scene(scene, html_document.as_ref(), 1.0, width, height);
        },
        width,
        height,
    );

    RgbaImage::from_vec(width, height, buf).ok_or_else(|| anyhow!("Could not create RgbaImage"))
}
