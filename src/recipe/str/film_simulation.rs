use crate::recipe::{
    str::{Error, ParseKey},
    FilmSimulation, MonochromaticFilter,
};
use std::str::FromStr;

impl FromStr for FilmSimulation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Provia" => Ok(FilmSimulation::ProviaStandard),
            "Velvia" => Ok(FilmSimulation::VelviaVivid),
            "Astia" => Ok(FilmSimulation::AstiaSoft),
            "Classic Chrome" => Ok(FilmSimulation::ClassicChrome),
            "Reala Ace" => Ok(FilmSimulation::RealaAce),
            "Pro Neg. Hi" => Ok(FilmSimulation::ProNegHi),
            "Pro Neg. Std" => Ok(FilmSimulation::ProNegStd),
            "Classic Negative" => Ok(FilmSimulation::ClassicNeg),
            "Eterna" => Ok(FilmSimulation::EternaCinema),
            "Eterna Bleach Bypass" => Ok(FilmSimulation::BleachBypass),
            "Acros" => Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Std,
            }),
            "Acros+Ye" => Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Yellow,
            }),
            "Acros +Ye" => Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Yellow,
            }),
            "Acros+R" => Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Red,
            }),
            "Acros +R" => Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Red,
            }),
            "Acros+G" => Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Green,
            }),
            "Acros +G" => Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Green,
            }),
            "Monochrome" => Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Std,
            }),
            "Monochrome+Ye" => Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Yellow,
            }),
            "Monochrome +Ye" => Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Yellow,
            }),
            "Monochrome+R" => Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Red,
            }),
            "Monochrome +R" => Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Red,
            }),
            "Monochrome+G" => Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Green,
            }),
            "Monochrome +G" => Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Green,
            }),
            "Sepia" => Ok(FilmSimulation::Sepia),
            _ => Err(Error::Parse {
                key: ParseKey::FilmSimulation,
                reason: format!("Invalid Film Simulation: {}", s),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::recipe::{
        str::{Error, ParseKey},
        FilmSimulation, MonochromaticFilter,
    };
    use std::str::FromStr;

    #[test]
    fn it_parses_provia_from_string() {
        let result = FilmSimulation::from_str("Provia");

        assert_eq!(result, Ok(FilmSimulation::ProviaStandard));
    }

    #[test]
    fn it_parses_velvia_from_string() {
        let result = FilmSimulation::from_str("Velvia");

        assert_eq!(result, Ok(FilmSimulation::VelviaVivid));
    }

    #[test]
    fn it_parses_astia_from_string() {
        let result = FilmSimulation::from_str("Astia");

        assert_eq!(result, Ok(FilmSimulation::AstiaSoft));
    }

    #[test]
    fn it_parses_classic_chrome_from_string() {
        let result = FilmSimulation::from_str("Classic Chrome");

        assert_eq!(result, Ok(FilmSimulation::ClassicChrome));
    }

    #[test]
    fn it_parses_reala_ace_from_string() {
        let result = FilmSimulation::from_str("Reala Ace");

        assert_eq!(result, Ok(FilmSimulation::RealaAce));
    }

    #[test]
    fn it_parses_classic_negative_from_string() {
        let result = FilmSimulation::from_str("Classic Negative");

        assert_eq!(result, Ok(FilmSimulation::ClassicNeg));
    }

    #[test]
    fn it_parses_eterna_from_string() {
        let result = FilmSimulation::from_str("Eterna");

        assert_eq!(result, Ok(FilmSimulation::EternaCinema));
    }

    #[test]
    fn it_parses_eterna_bleach_bypass_from_string() {
        let result = FilmSimulation::from_str("Eterna Bleach Bypass");

        assert_eq!(result, Ok(FilmSimulation::BleachBypass));
    }

    #[test]
    fn it_parses_acros_from_string() {
        let result = FilmSimulation::from_str("Acros");

        assert_eq!(
            result,
            Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Std
            })
        );
    }

    #[test]
    fn it_parses_acros_yellow_from_string() {
        let result = FilmSimulation::from_str("Acros+Ye");

        assert_eq!(
            result,
            Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Yellow
            })
        );

        let result = FilmSimulation::from_str("Acros +Ye");

        assert_eq!(
            result,
            Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Yellow
            })
        );
    }

    #[test]
    fn it_parses_acros_red_from_string() {
        let result = FilmSimulation::from_str("Acros+R");

        assert_eq!(
            result,
            Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Red
            })
        );

        let result = FilmSimulation::from_str("Acros +R");

        assert_eq!(
            result,
            Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Red
            })
        );
    }

    #[test]
    fn it_parses_acros_green_from_string() {
        let result = FilmSimulation::from_str("Acros+G");

        assert_eq!(
            result,
            Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Green
            })
        );

        let result = FilmSimulation::from_str("Acros +G");

        assert_eq!(
            result,
            Ok(FilmSimulation::Acros {
                filter: MonochromaticFilter::Green
            })
        );
    }

    #[test]
    fn it_parses_monochrome_from_string() {
        let result = FilmSimulation::from_str("Monochrome");

        assert_eq!(
            result,
            Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Std
            })
        );
    }

    #[test]
    fn it_parses_monochrome_yellow_from_string() {
        let result = FilmSimulation::from_str("Monochrome+Ye");

        assert_eq!(
            result,
            Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Yellow
            })
        );

        let result = FilmSimulation::from_str("Monochrome +Ye");

        assert_eq!(
            result,
            Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Yellow
            })
        );
    }

    #[test]
    fn it_parses_monochrome_red_from_string() {
        let result = FilmSimulation::from_str("Monochrome+R");

        assert_eq!(
            result,
            Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Red
            })
        );

        let result = FilmSimulation::from_str("Monochrome +R");

        assert_eq!(
            result,
            Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Red
            })
        );
    }

    #[test]
    fn it_parses_monochrome_green_from_string() {
        let result = FilmSimulation::from_str("Monochrome+G");

        assert_eq!(
            result,
            Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Green
            })
        );

        let result = FilmSimulation::from_str("Monochrome +G");

        assert_eq!(
            result,
            Ok(FilmSimulation::Monochrome {
                filter: MonochromaticFilter::Green
            })
        );
    }

    #[test]
    fn it_parses_sepia_from_string() {
        let result = FilmSimulation::from_str("Sepia");

        assert_eq!(result, Ok(FilmSimulation::Sepia));
    }

    #[test]
    fn it_fails_to_parse_invalid_string() {
        let result = FilmSimulation::from_str("Hello World");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::FilmSimulation,
                reason: "Invalid Film Simulation: Hello World".to_string()
            })
        );
    }
}
