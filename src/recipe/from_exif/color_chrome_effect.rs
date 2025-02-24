use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::{ColorChromeEffect, SettingStrength};
use log::trace;

impl FromExifData for ColorChromeEffect {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("ColorChromeEffect")?;

        trace!("ColorChromeEffect::from_exif: {:?}", exif);

        match exif.value().to_lowercase().as_str() {
            "strong" => Some(ColorChromeEffect {
                strength: SettingStrength::Strong,
            }),
            "weak" => Some(ColorChromeEffect {
                strength: SettingStrength::Weak,
            }),
            _ => Some(ColorChromeEffect {
                strength: SettingStrength::Off,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_strong_effect() {
        let exif: Vec<ExifData> = vec![ExifData::new("ColorChromeEffect", "Strong")];

        assert_eq!(
            ColorChromeEffect::from_exif(&exif),
            Some(ColorChromeEffect {
                strength: SettingStrength::Strong
            })
        );
    }

    #[test]
    fn it_parses_weak_effect() {
        let exif: Vec<ExifData> = vec![ExifData::new("ColorChromeEffect", "Weak")];

        assert_eq!(
            ColorChromeEffect::from_exif(&exif),
            Some(ColorChromeEffect {
                strength: SettingStrength::Weak
            })
        );
    }

    #[test]
    fn it_parses_off_effect() {
        let exif: Vec<ExifData> = vec![ExifData::new("ColorChromeEffect", "")];

        assert_eq!(
            ColorChromeEffect::from_exif(&exif),
            Some(ColorChromeEffect {
                strength: SettingStrength::Off
            })
        );
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "3")];

        assert_eq!(ColorChromeEffect::from_exif(&exif), None);
    }
}
