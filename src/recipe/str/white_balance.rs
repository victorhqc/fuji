use crate::recipe::{
    str::{Error, ParseKey},
    WBShift, WhiteBalance,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Display;
use std::str::FromStr;

impl WhiteBalance {
    pub fn to_string_no_shift(&self) -> String {
        match self {
            WhiteBalance::Auto { shift: _ } => "Auto".to_string(),
            WhiteBalance::AutoWhitePriority { shift: _ } => "AutoWhitePriority".to_string(),
            WhiteBalance::AutoAmbiencePriority { shift: _ } => "AutoAmbiencePriority".to_string(),
            WhiteBalance::Custom1 { shift: _ } => "Custom1".to_string(),
            WhiteBalance::Custom2 { shift: _ } => "Custom2".to_string(),
            WhiteBalance::Custom3 { shift: _ } => "Custom3".to_string(),
            WhiteBalance::Daylight { shift: _ } => "Daylight".to_string(),
            WhiteBalance::Cloudy { shift: _ } => "Cloudy".to_string(),
            WhiteBalance::FluorescentLight1 { shift: _ } => "FluorescentLight1".to_string(),
            WhiteBalance::FluorescentLight2 { shift: _ } => "FluorescentLight2".to_string(),
            WhiteBalance::FluorescentLight3 { shift: _ } => "FluorescentLight3".to_string(),
            WhiteBalance::Incandescent { shift: _ } => "Incandescent".to_string(),
            WhiteBalance::Underwater { shift: _ } => "Underwater".to_string(),
            WhiteBalance::Kelvin {
                temperature,
                shift: _,
            } => format!("{}K", temperature),
        }
    }
}

impl Display for WhiteBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WhiteBalance::Auto { shift } => write!(f, "Auto {}", shift),
            WhiteBalance::AutoWhitePriority { shift } => {
                write!(f, "AutoWhitePriority {}", shift)
            }
            WhiteBalance::AutoAmbiencePriority { shift } => {
                write!(f, "AutoAmbiencePriority {}", shift)
            }
            WhiteBalance::Custom1 { shift } => {
                write!(f, "Custom1 {}", shift)
            }
            WhiteBalance::Custom2 { shift } => {
                write!(f, "Custom2 {}", shift)
            }
            WhiteBalance::Custom3 { shift } => {
                write!(f, "Custom3 {}", shift)
            }
            WhiteBalance::Daylight { shift } => {
                write!(f, "Daylight {}", shift)
            }
            WhiteBalance::Cloudy { shift } => {
                write!(f, "Cloudy {}", shift)
            }
            WhiteBalance::FluorescentLight1 { shift } => {
                write!(f, "FluorescentLight1 {}", shift)
            }
            WhiteBalance::FluorescentLight2 { shift } => {
                write!(f, "FluorescentLight2 {}", shift)
            }
            WhiteBalance::FluorescentLight3 { shift } => {
                write!(f, "FluorescentLight3 {}", shift)
            }
            WhiteBalance::Incandescent { shift } => {
                write!(f, "Incandescent {}", shift)
            }
            WhiteBalance::Underwater { shift } => {
                write!(f, "Underwater {}", shift)
            }
            WhiteBalance::Kelvin { temperature, shift } => {
                write!(f, "{}K {}", temperature, shift)
            }
        }
    }
}

impl FromStr for WhiteBalance {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([0-9]+)K$").unwrap());
        let caps = RE.captures(s);

        if let Some(cap) = caps {
            let temp = cap.get(1).unwrap();
            let temp: i32 = temp.as_str().to_string().parse::<i32>().unwrap();

            return Ok(WhiteBalance::Kelvin {
                temperature: temp,
                shift: WBShift::default(),
            });
        };

        match s {
            "Auto" => Ok(WhiteBalance::Auto {
                shift: WBShift::default(),
            }),
            "AutoWhitePriority" => Ok(WhiteBalance::AutoWhitePriority {
                shift: WBShift::default(),
            }),
            "AutoAmbiencePriority" => Ok(WhiteBalance::AutoAmbiencePriority {
                shift: WBShift::default(),
            }),
            "Custom1" => Ok(WhiteBalance::Custom1 {
                shift: WBShift::default(),
            }),
            "Custom2" => Ok(WhiteBalance::Custom2 {
                shift: WBShift::default(),
            }),
            "Custom3" => Ok(WhiteBalance::Custom3 {
                shift: WBShift::default(),
            }),
            "Daylight" => Ok(WhiteBalance::Daylight {
                shift: WBShift::default(),
            }),
            "Cloudy" => Ok(WhiteBalance::Cloudy {
                shift: WBShift::default(),
            }),
            "FluorescentLight1" => Ok(WhiteBalance::FluorescentLight1 {
                shift: WBShift::default(),
            }),
            "FluorescentLight2" => Ok(WhiteBalance::FluorescentLight2 {
                shift: WBShift::default(),
            }),
            "FluorescentLight3" => Ok(WhiteBalance::FluorescentLight3 {
                shift: WBShift::default(),
            }),
            "Incandescent" => Ok(WhiteBalance::Incandescent {
                shift: WBShift::default(),
            }),
            "Underwater" => Ok(WhiteBalance::Underwater {
                shift: WBShift::default(),
            }),

            _ => Err(Error::Parse {
                key: ParseKey::WhiteBalance,
                reason: format!("Could not match WB to {}", s),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_wb_to_string() {
        let wb = WhiteBalance::Auto {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Auto R2, B-3");

        let wb = WhiteBalance::AutoWhitePriority {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "AutoWhitePriority R2, B-3");

        let wb = WhiteBalance::AutoAmbiencePriority {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "AutoAmbiencePriority R2, B-3");

        let wb = WhiteBalance::Custom1 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Custom1 R2, B-3");

        let wb = WhiteBalance::Custom2 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Custom2 R2, B-3");

        let wb = WhiteBalance::Custom3 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Custom3 R2, B-3");

        let wb = WhiteBalance::Daylight {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Daylight R2, B-3");

        let wb = WhiteBalance::Cloudy {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Cloudy R2, B-3");

        let wb = WhiteBalance::FluorescentLight1 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "FluorescentLight1 R2, B-3");

        let wb = WhiteBalance::FluorescentLight2 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "FluorescentLight2 R2, B-3");

        let wb = WhiteBalance::FluorescentLight3 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "FluorescentLight3 R2, B-3");

        let wb = WhiteBalance::Incandescent {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Incandescent R2, B-3");

        let wb = WhiteBalance::Underwater {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "Underwater R2, B-3");

        let wb = WhiteBalance::Kelvin {
            temperature: 9500,
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string();
        assert_eq!(&wb, "9500K R2, B-3");
    }

    #[test]
    fn it_parses_from_str() {
        assert_eq!(
            WhiteBalance::from_str("Auto"),
            Ok(WhiteBalance::Auto {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("AutoWhitePriority"),
            Ok(WhiteBalance::AutoWhitePriority {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("AutoAmbiencePriority"),
            Ok(WhiteBalance::AutoAmbiencePriority {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Custom1"),
            Ok(WhiteBalance::Custom1 {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Custom2"),
            Ok(WhiteBalance::Custom2 {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Custom3"),
            Ok(WhiteBalance::Custom3 {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Daylight"),
            Ok(WhiteBalance::Daylight {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Cloudy"),
            Ok(WhiteBalance::Cloudy {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("FluorescentLight1"),
            Ok(WhiteBalance::FluorescentLight1 {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("FluorescentLight2"),
            Ok(WhiteBalance::FluorescentLight2 {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("FluorescentLight3"),
            Ok(WhiteBalance::FluorescentLight3 {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Incandescent"),
            Ok(WhiteBalance::Incandescent {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("Underwater"),
            Ok(WhiteBalance::Underwater {
                shift: WBShift::default()
            })
        );

        assert_eq!(
            WhiteBalance::from_str("9360K"),
            Ok(WhiteBalance::Kelvin {
                temperature: 9360,
                shift: WBShift::default()
            })
        );
    }

    #[test]
    fn it_converts_to_string_no_shift() {
        let wb = WhiteBalance::Auto {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "Auto");

        let wb = WhiteBalance::AutoWhitePriority {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "AutoWhitePriority");

        let wb = WhiteBalance::AutoAmbiencePriority {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "AutoAmbiencePriority");

        let wb = WhiteBalance::Custom1 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "Custom1");

        let wb = WhiteBalance::Custom2 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "Custom2");

        let wb = WhiteBalance::Custom3 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "Custom3");

        let wb = WhiteBalance::Daylight {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "Daylight");

        let wb = WhiteBalance::Cloudy {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "Cloudy");

        let wb = WhiteBalance::FluorescentLight1 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "FluorescentLight1");

        let wb = WhiteBalance::FluorescentLight2 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "FluorescentLight2");

        let wb = WhiteBalance::FluorescentLight3 {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "FluorescentLight3");

        let wb = WhiteBalance::Incandescent {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "Incandescent");

        let wb = WhiteBalance::Underwater {
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "Underwater");

        let wb = WhiteBalance::Kelvin {
            temperature: 9500,
            shift: WBShift { red: 2, blue: -3 },
        }
        .to_string_no_shift();
        assert_eq!(&wb, "9500K");
    }
}
