//! The OG Image

use crate::{Error, Image, Text, background::Background};
use bon::Builder;
use taffy::{
    FlexDirection, Rect, Size, Style, TaffyTree,
    prelude::{TaffyMaxContent, auto, length},
};

/// The generated OG Image
#[derive(Builder, Debug)]
pub struct OgImage<'title_font, 'description_font> {
    /// Width of the generated OG Image
    #[builder(default = 1280)]
    width: u32,
    /// Height of the generated OG Image
    #[builder(default = 675)]
    height: u32,
    /// Title of the generated OG Image
    title: Text<'title_font>,
    /// Description of the generated OG Image
    description: Option<Text<'description_font>>,
    /// Background of the generated OG Image
    #[builder(default)]
    background: Background,
}

impl OgImage<'_, '_> {
    /// Create the OG Image
    pub fn create(self) -> Result<Image, Error> {
        let mut img = self.background.image(self.width, self.height)?;
        let mut tree = TaffyTree::<()>::new();

        let title = tree.new_leaf(Style {
            size: Size {
                width: auto(),
                height: length(200.0),
            },
            ..Default::default()
        })?;
        let description = tree.new_leaf(Style {
            size: Size {
                width: auto(),
                height: length(200.0),
            },
            ..Default::default()
        })?;

        let root_node = tree.new_with_children(
            Style {
                flex_direction: FlexDirection::Column,
                size: Size {
                    width: length(self.width as f32),
                    height: length(self.height as f32),
                },
                padding: Rect {
                    left: length(60.0),
                    right: length(0.0),
                    top: length(60.0),
                    bottom: length(0.0),
                },
                ..Default::default()
            },
            &[title, description],
        )?;

        tree.compute_layout(root_node, Size::MAX_CONTENT)?;

        let title = tree.layout(title)?.location;
        let description = tree.layout(description)?.location;

        self.title
            .draw(&mut img, title.x as i32, title.y as i32)
            .map_err(Error::InvalidTitleFont)?;
        if let Some(desc) = self.description {
            desc.draw(&mut img, description.x as i32, description.y as i32)
                .map_err(Error::InvalidDescriptionFont)?;
        }

        Ok(img)
    }
}
