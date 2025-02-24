use crate::exif::{ExifData, FromExifData};
use crate::recipe::{builder::SettingsBuilder, FilmSimulation, FujifilmRecipeDetails, TransSensor};

impl FromExifData for FujifilmRecipeDetails {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let film_simulation = FilmSimulation::from_exif(data)?;
        let sensor = TransSensor::from_exif(data)?;
        let builder = SettingsBuilder::from_exif(data)?;
        let settings = sensor.settings(builder);

        Some(FujifilmRecipeDetails {
            film_simulation,
            sensor,
            settings,
        })
    }
}
