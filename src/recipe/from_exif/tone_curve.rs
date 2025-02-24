use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::ToneCurve;
use log::trace;

impl FromExifData for ToneCurve {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let h_exif = data.find("HighlightTone")?;
        let s_exif = data.find("ShadowTone")?;

        trace!("ToneCurve::from_exif: H {:?}", h_exif);
        trace!("ToneCurve::from_exif: S {:?}", s_exif);

        let highlights: f64 = h_exif.try_into().ok()?;
        let shadows: f64 = s_exif.try_into().ok()?;

        Some(ToneCurve {
            highlights,
            shadows,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_tone_curve() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("HighlightTone", "1.5"),
            ExifData::new("ShadowTone", "-2"),
        ];

        assert_eq!(
            ToneCurve::from_exif(&exif),
            Some(ToneCurve {
                highlights: 1.5,
                shadows: -2.0,
            })
        );
    }

    #[test]
    fn it_does_not_parse_when_invalid_numbers() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("HighlightTone", "Hello"),
            ExifData::new("ShadowTone", "-2"),
        ];

        assert_eq!(ToneCurve::from_exif(&exif), None,);

        let exif: Vec<ExifData> = vec![
            ExifData::new("HighlightTone", "1.5"),
            ExifData::new("ShadowTone", "Bad"),
        ];

        assert_eq!(ToneCurve::from_exif(&exif), None,);
    }

    #[test]
    fn it_does_not_parse_when_missing_data() {
        let exif: Vec<ExifData> = vec![ExifData::new("HighlightTone", "1.5")];

        assert_eq!(ToneCurve::from_exif(&exif), None,);

        let exif: Vec<ExifData> = vec![ExifData::new("ShadowTone", "1.5")];

        assert_eq!(ToneCurve::from_exif(&exif), None,);
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "1.5")];

        assert_eq!(ToneCurve::from_exif(&exif), None,);
    }
}
