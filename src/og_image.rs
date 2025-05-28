//! The OG Image

use crate::{Error, Text, background::Background};
use bon::Builder;

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
    pub fn create(self) -> Result<(), Error> {
        let mut img = self.background.image(self.width, self.height)?;

        self.title.draw(&mut img).map_err(Error::InvalidTitleFont)?;
        if let Some(desc) = self.description {
            desc.draw(&mut img).map_err(Error::InvalidDescriptionFont)?;
        }

        Ok(())
    }
}
