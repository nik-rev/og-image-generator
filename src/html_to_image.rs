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
use std::sync::Arc;

pub async fn html_to_image(width: u32, height: u32, html: &str) -> Option<RgbaImage> {
    let (mut recv, callback) = MpscCallback::new();
    let callback = Arc::new(callback);
    let net = Arc::new(Provider::new(callback));

    let mut html_document = HtmlDocument::from_html(
        html,
        None,
        Vec::new(),
        Arc::clone(&net) as SharedProvider<Resource>,
        None,
        Arc::new(DummyNavigationProvider),
    );

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
    RgbaImage::from_vec(width, height, buf)
}
