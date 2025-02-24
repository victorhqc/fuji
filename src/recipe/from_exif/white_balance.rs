use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::{WBShift, WhiteBalance};
use log::trace;
use once_cell::sync::Lazy;
use regex::Regex;

impl FromExifData for WhiteBalance {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let shift = WBShift::from_exif(data).unwrap_or_default();
        let exif = data.find("WhiteBalance")?;

        trace!("WhiteBalance::from_exif: {:?}", exif);

        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"(?i:^(auto \(white priority\)|auto \(ambiance priority\)|auto|custom|custom2|custom3|kelvin|daylight fluorescent|day white fluorescent|white fluorescent|daylight|cloudy|incandescent|underwater)$)",
            ).unwrap()
        });

        let captures = RE.captures(exif.value())?;

        trace!("WhiteBalance Captures: {:?}", captures);

        match captures[1].to_lowercase().as_str() {
            "auto" => Some(WhiteBalance::Auto { shift }),
            "auto (white priority)" => Some(WhiteBalance::AutoWhitePriority { shift }),
            "auto (ambiance priority)" => Some(WhiteBalance::AutoAmbiencePriority { shift }),
            "custom" => Some(WhiteBalance::Custom1 { shift }),
            "custom2" => Some(WhiteBalance::Custom2 { shift }),
            "custom3" => Some(WhiteBalance::Custom3 { shift }),
            "daylight" => Some(WhiteBalance::Daylight { shift }),
            "cloudy" => Some(WhiteBalance::Cloudy { shift }),
            "daylight fluorescent" => Some(WhiteBalance::FluorescentLight1 { shift }),
            "day white fluorescent" => Some(WhiteBalance::FluorescentLight2 { shift }),
            "white fluorescent" => Some(WhiteBalance::FluorescentLight3 { shift }),
            "incandescent" => Some(WhiteBalance::Incandescent { shift }),
            "underwater" => Some(WhiteBalance::Underwater { shift }),
            "kelvin" => {
                if let Some(exif) = data.find("ColorTemperature") {
                    trace!("WhiteBalance::Temperature {:?}", exif.value());

                    if let Ok(temperature) = exif.value().parse::<i32>() {
                        Some(WhiteBalance::Kelvin { temperature, shift })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_white_balance_auto() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Auto")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Auto {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Auto"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Auto {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_auto_white_priority() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Auto (White Priority)")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::AutoWhitePriority {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Auto (White Priority)"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::AutoWhitePriority {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_auto_ambience_priority() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Auto (Ambiance Priority)")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::AutoAmbiencePriority {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Auto (Ambiance Priority)"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::AutoAmbiencePriority {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_custom() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Custom")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Custom1 {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Custom"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Custom1 {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_custom2() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Custom2")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Custom2 {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Custom2"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Custom2 {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_custom3() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Custom3")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Custom3 {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Custom3"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Custom3 {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_daylight() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Daylight")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Daylight {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Daylight"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Daylight {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_cloudy() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Cloudy")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Cloudy {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Cloudy"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Cloudy {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_fluorescent1() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Daylight Fluorescent")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::FluorescentLight1 {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Daylight Fluorescent"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::FluorescentLight1 {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_fluorescent2() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Day White Fluorescent")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::FluorescentLight2 {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Day White Fluorescent"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::FluorescentLight2 {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_fluorescent3() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "White Fluorescent")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::FluorescentLight3 {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "White Fluorescent"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::FluorescentLight3 {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_incandescent() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Incandescent")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Incandescent {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Incandescent"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Incandescent {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_underwater() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalance", "Underwater")];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Underwater {
                shift: WBShift { red: 0, blue: 0 }
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Underwater"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Underwater {
                shift: WBShift { red: 2, blue: -3 }
            })
        );
    }

    #[test]
    fn it_parses_white_balance_kelvin() {
        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Kelvin"),
            ExifData::new("ColorTemperature", "6500"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Kelvin {
                shift: WBShift { red: 0, blue: 0 },
                temperature: 6500,
            })
        );

        let exif: Vec<ExifData> = vec![
            ExifData::new("WhiteBalance", "Kelvin"),
            ExifData::new("WhiteBalanceFineTune", "red 40, blue -60"),
            ExifData::new("ColorTemperature", "6500"),
        ];
        assert_eq!(
            WhiteBalance::from_exif(&exif),
            Some(WhiteBalance::Kelvin {
                shift: WBShift { red: 2, blue: -3 },
                temperature: 6500,
            })
        );
    }

    #[test]
    fn it_does_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "Auto")];
        assert_eq!(WhiteBalance::from_exif(&exif), None,);
    }
}
