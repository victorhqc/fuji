use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::DRangePriority;
use log::trace;

impl FromExifData for DRangePriority {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let value: String = if let Some(exif) = data.find("DRangePriorityAuto") {
            trace!("DRangePriority::from_exif: {:?}", exif);

            exif.value().to_string()
        } else if let Some(exif) = data.find("DRangePriority") {
            trace!("DRangePriority::from_exif: {:?}", exif);

            exif.value().to_string()
        } else {
            "".to_string()
        };

        match value.to_lowercase().as_str() {
            "strong" => Some(DRangePriority::Strong),
            "weak" => Some(DRangePriority::Weak),
            "auto" => Some(DRangePriority::Auto),
            _ => Some(DRangePriority::Off),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_auto_priority() {
        let exif: Vec<ExifData> = vec![ExifData::new("DRangePriorityAuto", "auto")];

        assert_eq!(DRangePriority::from_exif(&exif), Some(DRangePriority::Auto));
    }

    #[test]
    fn it_parses_strong_priority() {
        let exif: Vec<ExifData> = vec![ExifData::new("DRangePriority", "strong")];

        assert_eq!(
            DRangePriority::from_exif(&exif),
            Some(DRangePriority::Strong)
        );
    }

    #[test]
    fn it_parses_weak_priority() {
        let exif: Vec<ExifData> = vec![ExifData::new("DRangePriority", "weak")];

        assert_eq!(DRangePriority::from_exif(&exif), Some(DRangePriority::Weak));
    }

    #[test]
    fn it_parses_as_off_when_missing() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "3")];

        assert_eq!(DRangePriority::from_exif(&exif), Some(DRangePriority::Off));
    }
}
