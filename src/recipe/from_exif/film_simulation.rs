use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::{FilmSimulation, MonochromaticFilter};
use log::trace;
use once_cell::sync::Lazy;
use regex::Regex;

impl FromExifData for FilmSimulation {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = if let Some(v) = data.find("FilmMode") {
            v
        } else if let Some(v) = data.find("Saturation") {
            v
        } else {
            return None;
        };

        trace!("FilmSimulation::from_exif: {:?}", exif);

        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
            r"(?i:provia|velvia|astia|classic chrome|classic neg|nostalgic neg|acros yellow filter|acros red filter|acros green filter|acros|eterna|b&w sepia|b&w yellow filter|b&w red filter|b&w green filter|b&w|reala ace|pro neg\.? std|pro neg\.? standard|pro neg\.? hi|bleach bypass)",
        ).unwrap()
        });

        let captures = RE.captures(exif.value())?;

        trace!("FilmSimulation Captures: {:?}", captures);

        match captures[0].to_lowercase().as_str() {
            "provia" => Some(FilmSimulation::ProviaStandard),
            "velvia" => Some(FilmSimulation::VelviaVivid),
            "astia" => Some(FilmSimulation::AstiaSoft),
            "classic chrome" => Some(FilmSimulation::ClassicChrome),
            "classic neg" => Some(FilmSimulation::ClassicNeg),
            "nostalgic neg" => Some(FilmSimulation::NostalgicNeg),
            "pro neg. std" => Some(FilmSimulation::ProNegStd),
            "pro neg std" => Some(FilmSimulation::ProNegStd),
            "pro neg. standard" => Some(FilmSimulation::ProNegStd),
            "pro neg standard" => Some(FilmSimulation::ProNegStd),
            "pro neg. hi" => Some(FilmSimulation::ProNegHi),
            "pro neg hi" => Some(FilmSimulation::ProNegHi),
            "eterna" => Some(FilmSimulation::EternaCinema),
            "bleach bypass" => Some(FilmSimulation::BleachBypass),
            "acros yellow filter" => Some(FilmSimulation::Acros {
                filter: MonochromaticFilter::Yellow,
            }),
            "acros red filter" => Some(FilmSimulation::Acros {
                filter: MonochromaticFilter::Red,
            }),
            "acros green filter" => Some(FilmSimulation::Acros {
                filter: MonochromaticFilter::Green,
            }),
            "acros" => Some(FilmSimulation::Acros {
                filter: MonochromaticFilter::Std,
            }),
            "b&w" => Some(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Std,
            }),
            "b&w yellow filter" => Some(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Yellow,
            }),
            "b&w red filter" => Some(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Red,
            }),
            "b&w green filter" => Some(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Green,
            }),
            "b&w sepia" => Some(FilmSimulation::Sepia),
            "reala ace" => Some(FilmSimulation::RealaAce),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_provia() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Provia")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::ProviaStandard)
        );
    }

    #[test]
    fn it_parses_velvia() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Velvia")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::VelviaVivid)
        );
    }

    #[test]
    fn it_parses_astia() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Astia")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::AstiaSoft)
        );
    }

    #[test]
    fn it_parses_classic_chrome() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Classic Chrome")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::ClassicChrome)
        );
    }

    #[test]
    fn it_parses_reala_ace() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Reala Ace")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::RealaAce)
        );
    }

    #[test]
    fn it_parses_classic_negative() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Classic Negative")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::ClassicNeg)
        );
    }

    #[test]
    fn it_parses_nostalgic_negative() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Nostalgic Negative")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::NostalgicNeg)
        );
    }

    #[test]
    fn it_parses_pro_neg_std() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Pro Neg Std")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::ProNegStd)
        );

        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Pro Neg. Std")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::ProNegStd)
        );

        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Pro Neg. Standard")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::ProNegStd)
        );

        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Pro Neg Standard")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::ProNegStd)
        );
    }

    #[test]
    fn it_parses_pro_neg_high() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Pro Neg Hi")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::ProNegHi)
        );

        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Pro Neg. Hi")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::ProNegHi)
        );
    }

    #[test]
    fn it_parses_eterna() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Eterna")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::EternaCinema)
        );
    }

    #[test]
    fn it_parses_bleach_bypass() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Bleach Bypass")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::BleachBypass)
        );
    }

    #[test]
    fn it_parses_acros_standard() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Acros")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::Acros {
                filter: MonochromaticFilter::Std
            })
        );
    }

    #[test]
    fn it_parses_acros_yellow() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Acros Yellow Filter")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::Acros {
                filter: MonochromaticFilter::Yellow
            })
        );
    }

    #[test]
    fn it_parses_acros_green() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Acros Green Filter")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::Acros {
                filter: MonochromaticFilter::Green
            })
        );
    }

    #[test]
    fn it_parses_acros_red() {
        let exif: Vec<ExifData> = vec![ExifData::new("FilmMode", "Acros Red Filter")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::Acros {
                filter: MonochromaticFilter::Red
            })
        );
    }

    #[test]
    fn it_parses_monochrome_standard() {
        let exif: Vec<ExifData> = vec![ExifData::new("Saturation", "B&W")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Std
            })
        );
    }

    #[test]
    fn it_parses_monochrome_standard_yellow() {
        let exif: Vec<ExifData> = vec![ExifData::new("Saturation", "B&W Yellow Filter")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Yellow
            })
        );
    }

    #[test]
    fn it_parses_monochrome_green() {
        let exif: Vec<ExifData> = vec![ExifData::new("Saturation", "B&W Green Filter")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Green
            })
        );
    }

    #[test]
    fn it_parses_monochrome_red() {
        let exif: Vec<ExifData> = vec![ExifData::new("Saturation", "B&W Red Filter")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Red
            })
        );
    }

    #[test]
    fn it_parses_sepia() {
        let exif: Vec<ExifData> = vec![ExifData::new("Saturation", "B&W Sepia")];

        assert_eq!(
            FilmSimulation::from_exif(&exif),
            Some(FilmSimulation::Sepia)
        );
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "3")];

        assert_eq!(FilmSimulation::from_exif(&exif), None);
    }
}
