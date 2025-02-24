use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::FujifilmRecipeDetails;
use snafu::prelude::*;

pub fn from_exif(data: &[ExifData]) -> Result<Option<FujifilmRecipeDetails>, Error> {
    let maker = data.find("Make").context(MakeNotFoundSnafu)?;

    if maker.value().to_lowercase() == "fujifilm" {
        let recipe_details =
            FujifilmRecipeDetails::from_exif(data).context(FujifilmRecipeDetailsSnafu)?;

        return Ok(Some(recipe_details));
    }

    Ok(None)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to find Camera Maker"))]
    MakeNotFound,

    #[snafu(display("Could not find Fujifilm Recipe details in EXIF"))]
    FujifilmRecipeDetails,
}
