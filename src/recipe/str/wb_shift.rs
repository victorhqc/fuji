use crate::recipe::{
    str::{Error, ParseKey},
    WBShift,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Display;
use std::str::FromStr;

impl Display for WBShift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R{}, B{}", self.red, self.blue)
    }
}

impl FromStr for WBShift {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::Parse {
                key: ParseKey::WhiteBalanceShift,
                reason: "Invalid Empty String".to_string(),
            });
        }
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([0-9+\-.]+)").unwrap());

        let shifts: Result<Vec<i32>, Error> = String::from(s)
            .split(",")
            .map(|val| {
                let caps = RE.captures(val);

                let val: &str = if let Some(caps) = caps {
                    caps.get(1).map_or("", |m| m.as_str())
                } else {
                    ""
                };

                if val.is_empty() {
                    return Err(Error::Parse {
                        key: ParseKey::WhiteBalanceShift,
                        reason: format!("Invalid Empty value in: {}", s),
                    });
                }

                val.parse::<i32>().map_err(|_| Error::Parse {
                    key: ParseKey::WhiteBalanceShift,
                    reason: format!("{} is not a valid integer", val),
                })
            })
            .collect();

        let shifts = shifts?;

        if shifts.len() != 2 {
            return Err(Error::Parse {
                key: ParseKey::WhiteBalanceShift,
                reason: format!(
                    "Wrong amount of values, should be 2 but got: {}",
                    shifts.len(),
                ),
            });
        }

        let red: &i32 = shifts.first().unwrap_or(&0);
        let blue: &i32 = shifts.get(1).unwrap_or(&0);

        Ok(WBShift {
            red: red.to_owned(),
            blue: blue.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::str::{Error, ParseKey};

    #[test]
    fn it_parses_a_valid_wb_shift() {
        let wb_shift = WBShift { red: 2, blue: 3 }.to_string();

        assert_eq!(&wb_shift, "R2, B3");
        assert_eq!(
            WBShift::from_str(&wb_shift),
            Ok(WBShift { red: 2, blue: 3 })
        );
    }

    #[test]
    fn it_supports_negative_values() {
        let wb_shift = WBShift { red: -2, blue: 3 }.to_string();

        assert_eq!(&wb_shift, "R-2, B3");
        assert_eq!(
            WBShift::from_str(&wb_shift),
            Ok(WBShift { red: -2, blue: 3 })
        );

        let wb_shift = WBShift { red: 2, blue: -3 }.to_string();

        assert_eq!(&wb_shift, "R2, B-3");
        assert_eq!(
            WBShift::from_str(&wb_shift),
            Ok(WBShift { red: 2, blue: -3 })
        );
    }

    #[test]
    fn it_fails_when_empty() {
        let result = WBShift::from_str("");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::WhiteBalanceShift,
                reason: "Invalid Empty String".to_string(),
            })
        );
    }

    #[test]
    fn it_fails_empty_half() {
        let result = WBShift::from_str("12,");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::WhiteBalanceShift,
                reason: "Invalid Empty value in: 12,".to_string(),
            })
        );

        let result = WBShift::from_str(",12");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::WhiteBalanceShift,
                reason: "Invalid Empty value in: ,12".to_string(),
            })
        );
    }

    #[test]
    fn it_fails_when_missing_values() {
        let result = WBShift::from_str("1");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::WhiteBalanceShift,
                reason: "Wrong amount of values, should be 2 but got: 1".to_string(),
            })
        );
    }

    #[test]
    fn it_fails_when_more_values() {
        let result = WBShift::from_str("1,3,4");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::WhiteBalanceShift,
                reason: "Wrong amount of values, should be 2 but got: 3".to_string(),
            })
        );
    }

    #[test]
    fn it_fails_when_values_are_not_int() {
        let result = WBShift::from_str("R2.5, B3");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::WhiteBalanceShift,
                reason: "2.5 is not a valid integer".to_string(),
            })
        );

        let result = WBShift::from_str("2, 3.2");

        assert_eq!(
            result,
            Err(Error::Parse {
                key: ParseKey::WhiteBalanceShift,
                reason: "3.2 is not a valid integer".to_string(),
            })
        );
    }
}
