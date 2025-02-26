use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::{MonochromaticColor, MonochromaticColorShift};
use log::trace;

impl FromExifData for MonochromaticColor {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        // TODO: Adjust Toning for X-Trans III Cameras (X-T3, X-T30, etc.)
        let exif_adjustment = data.find("BWAdjustment")?;
        let exif_magenta = data.find("BWMagentaGreen")?;

        trace!("MonochromaticColor::from_exif: WC {:?}", exif_adjustment);
        trace!("MonochromaticColor::from_exif: MG {:?}", exif_magenta);

        let wc: i64 = exif_adjustment.try_into().ok()?;
        let mg: i64 = exif_magenta.try_into().ok()?;

        Some(MonochromaticColor::ColorShift {
            shift: MonochromaticColorShift { wc, mg },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_monochromatic_color() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("BWAdjustment", "3"),
            ExifData::new("BWMagentaGreen", "-5"),
        ];

        assert_eq!(
            MonochromaticColor::from_exif(&exif),
            Some(MonochromaticColor::ColorShift {
                shift: MonochromaticColorShift { wc: 3, mg: -5 },
            })
        );
    }

    #[test]
    fn it_does_not_parse_when_invalid_numbers() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("BWAdjustment", "hello"),
            ExifData::new("BWMagentaGreen", "-5"),
        ];

        assert_eq!(MonochromaticColor::from_exif(&exif), None,);

        let exif: Vec<ExifData> = vec![
            ExifData::new("BWAdjustment", "3"),
            ExifData::new("BWMagentaGreen", "bad"),
        ];

        assert_eq!(MonochromaticColor::from_exif(&exif), None,);
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "3")];

        assert_eq!(MonochromaticColor::from_exif(&exif), None,);
    }
}
