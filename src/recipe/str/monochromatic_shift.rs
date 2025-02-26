use crate::recipe::{
    str::{Error, ParseKey},
    MonochromaticColorShift,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl Display for MonochromaticColorShift {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WC {}, MG {}", self.wc, self.mg)
    }
}

impl FromStr for MonochromaticColorShift {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::Parse {
                key: ParseKey::MonochromaticColorShift,
                reason: "Invalid Empty String".to_string(),
            });
        }
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"(?x)
                (?P<WC>WC\s([0-9+\-.]+))
                ,\s?
                (?P<MG>MG\s([0-9+\-.]+))
            ",
            )
            .unwrap()
        });

        let caps = if let Some(caps) = RE.captures(s) {
            caps
        } else {
            return Err(Error::Parse {
                key: ParseKey::MonochromaticColorShift,
                reason: format!("Invalid String: {}", s),
            });
        };

        let parsed = (
            caps.get(2).unwrap().as_str().parse::<i64>(),
            caps.get(4).unwrap().as_str().parse::<i64>(),
        );

        if let (Ok(wc), Ok(mg)) = parsed {
            Ok(MonochromaticColorShift { wc, mg })
        } else {
            Err(Error::Parse {
                key: ParseKey::MonochromaticColorShift,
                reason: format!("Invalid number in: {}", s),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_to_string() {
        let result = MonochromaticColorShift { wc: 2, mg: 3 }.to_string();

        assert_eq!(&result, "WC 2, MG 3");
    }

    #[test]
    fn it_parses_negative_numbers_to_string() {
        let result = MonochromaticColorShift { wc: -5, mg: -7 }.to_string();

        assert_eq!(&result, "WC -5, MG -7");
    }

    #[test]
    fn it_parses_from_string() {
        let result = MonochromaticColorShift::from_str("WC 2, MG -3");

        assert_eq!(result, Ok(MonochromaticColorShift { wc: 2, mg: -3 }));
    }

    #[test]
    fn it_fails_to_parse_empty_string() {
        let result = MonochromaticColorShift::from_str("");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::MonochromaticColorShift,
                reason: "Invalid Empty String".to_string(),
            })
        );
    }

    #[test]
    fn it_fails_to_parse_invalid_string() {
        let result = MonochromaticColorShift::from_str("MG 2, WC -3");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::MonochromaticColorShift,
                reason: "Invalid String: MG 2, WC -3".to_string(),
            })
        );
    }

    #[test]
    fn it_fails_to_parse_invalid_numbers() {
        let result = MonochromaticColorShift::from_str("WC 3--1, MG 2");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::MonochromaticColorShift,
                reason: "Invalid number in: WC 3--1, MG 2".to_string(),
            })
        );
    }
}
