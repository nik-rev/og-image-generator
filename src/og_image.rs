//! The OG Image

use crate::{
    Text,
    background::{self, Background},
};
use ab_glyph::InvalidFont;
use bon::Builder;

/// The generated OG Image
#[derive(Builder)]
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

/// Error trying to create the OG Image
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to get the background
    #[error(transparent)]
    Background(background::Error),
    /// Font for title is not valid
    #[error(transparent)]
    InvalidTitleFont(InvalidFont),
    /// Font for description is not valid
    #[error(transparent)]
    InvalidDescriptionFont(InvalidFont),
}

impl OgImage<'_, '_> {
    /// Create the OG Image
    pub fn create(self) -> Result<(), Error> {
        let mut img = self
            .background
            .image(self.width, self.height)
            .map_err(Error::Background)?;

        self.title.draw(&mut img).map_err(Error::InvalidTitleFont)?;
        if let Some(desc) = self.description {
            desc.draw(&mut img).map_err(Error::InvalidDescriptionFont)?;
        }

        Ok(())
    }
}
