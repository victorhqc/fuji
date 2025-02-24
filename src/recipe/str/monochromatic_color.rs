use crate::recipe::{
    str::{Error, ParseKey},
    MonochromaticColor, MonochromaticColorShift,
};
use crate::utils::str::AddSign;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl Display for MonochromaticColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MonochromaticColor::Strength { value } => {
                write!(f, "{}", &value.add_sign())
            }
            MonochromaticColor::ColorShift { shift } => {
                write!(f, "{}", shift)
            }
        }
    }
}

impl FromStr for MonochromaticColor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::Parse {
                key: ParseKey::MonochromaticColor,
                reason: "Invalid Empty String".to_string(),
            });
        }
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<str>^([0-9+\-.]+)$)").unwrap());
        let caps = RE.captures(s);

        if let Some(cap) = caps {
            let str = &cap["str"].parse::<i64>();
            return if let Ok(str) = str {
                Ok(MonochromaticColor::Strength { value: *str })
            } else {
                Err(Error::Parse {
                    key: ParseKey::MonochromaticColor,
                    reason: format!("Invalid Number: {}", &cap["str"]),
                })
            };
        }

        match MonochromaticColorShift::from_str(s) {
            Ok(s) => Ok(MonochromaticColor::ColorShift { shift: s }),
            Err(e) => Err(Error::Parse {
                key: ParseKey::MonochromaticColor,
                reason: e.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::MonochromaticColorShift;

    #[test]
    fn it_parses_strength_to_string() {
        let result = MonochromaticColor::Strength { value: 3 }.to_string();
        assert_eq!(result, "+3");

        let result = MonochromaticColor::Strength { value: -5 }.to_string();
        assert_eq!(result, "-5");

        let result = MonochromaticColor::Strength { value: 0 }.to_string();
        assert_eq!(result, "0");
    }

    #[test]
    fn it_parses_shift_to_string() {
        let result = MonochromaticColor::ColorShift {
            shift: MonochromaticColorShift { wc: 2, mg: -3 },
        }
        .to_string();

        assert_eq!(result, "WC 2, MG -3");
    }

    #[test]
    fn it_parses_strength_from_string() {
        let result = MonochromaticColor::from_str("+3");

        assert_eq!(result, Ok(MonochromaticColor::Strength { value: 3 }));

        let result = MonochromaticColor::from_str("-5");

        assert_eq!(result, Ok(MonochromaticColor::Strength { value: -5 }));
    }

    #[test]
    fn it_fails_to_parse_on_empty_string() {
        let result = MonochromaticColor::from_str("");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::MonochromaticColor,
                reason: "Invalid Empty String".to_string(),
            })
        );
    }

    #[test]
    fn it_fails_to_parse_strength_when_invalid_number() {
        let result = MonochromaticColor::from_str("++3");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::MonochromaticColor,
                reason: "Invalid Number: ++3".to_string(),
            })
        );

        let result = MonochromaticColor::from_str("+3.5");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::MonochromaticColor,
                reason: "Invalid Number: +3.5".to_string(),
            })
        );
    }

    #[test]
    fn it_parses_shift_from_string() {
        let result = MonochromaticColor::from_str("WC 2, MG -3");

        assert_eq!(
            result,
            Ok(MonochromaticColor::ColorShift {
                shift: MonochromaticColorShift { wc: 2, mg: -3 },
            })
        );
    }

    #[test]
    fn it_fails_to_parse_shift_from_string() {
        let result = MonochromaticColor::from_str("WC 2, MG-3");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::MonochromaticColor,
                reason: "Failed to Parse MonochromaticColorShift: Invalid String: WC 2, MG-3"
                    .to_string(),
            })
        );
    }
}
