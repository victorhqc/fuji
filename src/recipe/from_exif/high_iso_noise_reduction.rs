use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::HighISONoiseReduction;
use log::trace;

impl FromExifData for HighISONoiseReduction {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("NoiseReduction")?;

        trace!("HighISONoiseReduction::from_exif: {:?}", exif);

        let value: i64 = exif.try_into().ok()?;

        Some(HighISONoiseReduction { value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_positive_numbers() {
        let exif: Vec<ExifData> = vec![ExifData::new("NoiseReduction", "3")];

        assert_eq!(
            HighISONoiseReduction::from_exif(&exif),
            Some(HighISONoiseReduction { value: 3 })
        );
    }

    #[test]
    fn it_parses_negative_numbers() {
        let exif: Vec<ExifData> = vec![ExifData::new("NoiseReduction", "-3")];

        assert_eq!(
            HighISONoiseReduction::from_exif(&exif),
            Some(HighISONoiseReduction { value: -3 })
        );
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "3")];

        assert_eq!(HighISONoiseReduction::from_exif(&exif), None);
    }
}
