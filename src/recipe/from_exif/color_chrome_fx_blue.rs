use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::{ColorChromeEffectFxBlue, SettingStrength};
use log::trace;

impl FromExifData for ColorChromeEffectFxBlue {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("ColorChromeFXBlue")?;

        trace!("ColorChromeEffectFxBlue::from_exif: {:?}", exif);

        match exif.value().to_lowercase().as_str() {
            "strong" => Some(ColorChromeEffectFxBlue {
                strength: SettingStrength::Strong,
            }),
            "weak" => Some(ColorChromeEffectFxBlue {
                strength: SettingStrength::Weak,
            }),
            _ => Some(ColorChromeEffectFxBlue {
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
        let exif: Vec<ExifData> = vec![ExifData::new("ColorChromeFXBlue", "Strong")];

        assert_eq!(
            ColorChromeEffectFxBlue::from_exif(&exif),
            Some(ColorChromeEffectFxBlue {
                strength: SettingStrength::Strong
            })
        );
    }

    #[test]
    fn it_parses_weak_effect() {
        let exif: Vec<ExifData> = vec![ExifData::new("ColorChromeFXBlue", "Weak")];

        assert_eq!(
            ColorChromeEffectFxBlue::from_exif(&exif),
            Some(ColorChromeEffectFxBlue {
                strength: SettingStrength::Weak
            })
        );
    }

    #[test]
    fn it_parses_off_effect() {
        let exif: Vec<ExifData> = vec![ExifData::new("ColorChromeFXBlue", "")];

        assert_eq!(
            ColorChromeEffectFxBlue::from_exif(&exif),
            Some(ColorChromeEffectFxBlue {
                strength: SettingStrength::Off
            })
        );
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "3")];

        assert_eq!(ColorChromeEffectFxBlue::from_exif(&exif), None);
    }
}
