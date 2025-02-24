use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::{GrainEffect, GrainSize, GrainStrength};
use log::trace;

impl FromExifData for GrainEffect {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let strength_exif = data.find("GrainEffectRoughness")?;

        trace!("GrainStrength::from_exif: {:?}", strength_exif);

        let size_exif = data.find("GrainEffectSize").unwrap_or_default();

        trace!("GrainSize::from_exif: {:?}", size_exif);

        let grain_size = match size_exif.value().to_lowercase().as_str() {
            "small" => Some(GrainSize::Small),
            "large" => Some(GrainSize::Large),
            _ => None,
        };

        if let Some(grain_size) = grain_size {
            match strength_exif.value().to_lowercase().as_str() {
                "strong" => Some(GrainEffect::StrengthAndSize {
                    strength: GrainStrength::Strong,
                    size: grain_size,
                }),
                "weak" => Some(GrainEffect::StrengthAndSize {
                    strength: GrainStrength::Weak,
                    size: grain_size,
                }),
                _ => Some(GrainEffect::Off),
            }
        } else {
            match strength_exif.value().to_lowercase().as_str() {
                "strong" => Some(GrainEffect::OnlyStrength {
                    strength: GrainStrength::Strong,
                }),
                "weak" => Some(GrainEffect::OnlyStrength {
                    strength: GrainStrength::Weak,
                }),
                _ => Some(GrainEffect::Off),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_strong_grain() {
        let exif: Vec<ExifData> = vec![ExifData::new("GrainEffectRoughness", "Strong")];

        assert_eq!(
            GrainEffect::from_exif(&exif),
            Some(GrainEffect::OnlyStrength {
                strength: GrainStrength::Strong
            })
        );
    }

    #[test]
    fn it_parses_weak_grain() {
        let exif: Vec<ExifData> = vec![ExifData::new("GrainEffectRoughness", "Weak")];

        assert_eq!(
            GrainEffect::from_exif(&exif),
            Some(GrainEffect::OnlyStrength {
                strength: GrainStrength::Weak
            })
        );
    }

    #[test]
    fn it_parses_small_strong_grain() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("GrainEffectRoughness", "Strong"),
            ExifData::new("GrainEffectSize", "Small"),
        ];

        assert_eq!(
            GrainEffect::from_exif(&exif),
            Some(GrainEffect::StrengthAndSize {
                size: GrainSize::Small,
                strength: GrainStrength::Strong
            })
        );
    }

    #[test]
    fn it_parses_small_weak_grain() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("GrainEffectRoughness", "Weak"),
            ExifData::new("GrainEffectSize", "Small"),
        ];

        assert_eq!(
            GrainEffect::from_exif(&exif),
            Some(GrainEffect::StrengthAndSize {
                size: GrainSize::Small,
                strength: GrainStrength::Weak
            })
        );
    }

    #[test]
    fn it_parses_large_strong_grain() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("GrainEffectRoughness", "Strong"),
            ExifData::new("GrainEffectSize", "Large"),
        ];

        assert_eq!(
            GrainEffect::from_exif(&exif),
            Some(GrainEffect::StrengthAndSize {
                size: GrainSize::Large,
                strength: GrainStrength::Strong
            })
        );
    }

    #[test]
    fn it_parses_large_weak_grain() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("GrainEffectRoughness", "Weak"),
            ExifData::new("GrainEffectSize", "Large"),
        ];

        assert_eq!(
            GrainEffect::from_exif(&exif),
            Some(GrainEffect::StrengthAndSize {
                size: GrainSize::Large,
                strength: GrainStrength::Weak
            })
        );
    }

    #[test]
    fn it_does_not_parse_when_only_size() {
        let exif: Vec<ExifData> = vec![ExifData::new("GrainEffectSize", "Large")];

        assert_eq!(GrainEffect::from_exif(&exif), None);
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "Strong")];

        assert_eq!(GrainEffect::from_exif(&exif), None);
    }
}
