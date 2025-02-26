use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::FujifilmRecipeDetails;
use snafu::prelude::*;

/// Reads a Fujifilm Recipe from exif data.
///
/// # Example
///
/// ```
/// use fuji::exiftool::spawn;
/// use fuji::recipe::read;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Read metadata from a Fujifilm image
/// let path = std::path::Path::new("tests/img/DSCF5230.JPG");
/// let metadata = spawn::read_metadata(&path, None)?;
///
/// // Parse the Fujifilm recipe from the metadata
/// let recipe = read::from_exif(&metadata)?;
///
/// // Recipe will contain details like FilmSimulation, WhiteBalance, etc.
/// if let Some(details) = recipe {
///     println!("Film Simulation: {:?}", details.film_simulation);
///     println!("White Balance: {:?}", details.settings);
/// }
/// # Ok(())
/// # }
/// ```
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
