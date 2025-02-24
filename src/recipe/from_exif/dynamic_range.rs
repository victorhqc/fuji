use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::DynamicRange;
use log::trace;

impl FromExifData for DynamicRange {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        if let Some(exif) = data.find("DynamicRangeSetting") {
            trace!("DynamicRange::from_exif: {:?}", exif);

            return Some(DynamicRange::Auto);
        };

        let exif = data.find("DevelopmentDynamicRange")?;

        trace!("DynamicRange::from_exif: {:?}", exif);

        match exif.value().to_lowercase().as_str() {
            "100" => Some(DynamicRange::DR100),
            "200" => Some(DynamicRange::DR200),
            "400" => Some(DynamicRange::DR400),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_auto_dynamic_range() {
        let exif: Vec<ExifData> = vec![ExifData::new("DynamicRangeSetting", "auto")];

        assert_eq!(DynamicRange::from_exif(&exif), Some(DynamicRange::Auto));
    }

    #[test]
    fn it_parses_dr100_dynamic_range() {
        let exif: Vec<ExifData> = vec![ExifData::new("DevelopmentDynamicRange", "100")];

        assert_eq!(DynamicRange::from_exif(&exif), Some(DynamicRange::DR100));
    }

    #[test]
    fn it_parses_dr200_dynamic_range() {
        let exif: Vec<ExifData> = vec![ExifData::new("DevelopmentDynamicRange", "200")];

        assert_eq!(DynamicRange::from_exif(&exif), Some(DynamicRange::DR200));
    }

    #[test]
    fn it_parses_dr400_dynamic_range() {
        let exif: Vec<ExifData> = vec![ExifData::new("DevelopmentDynamicRange", "400")];

        assert_eq!(DynamicRange::from_exif(&exif), Some(DynamicRange::DR400));
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "400")];

        assert_eq!(DynamicRange::from_exif(&exif), None);
    }
}
