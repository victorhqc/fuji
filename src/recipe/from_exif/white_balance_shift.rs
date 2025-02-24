use crate::exif::{ExifData, FindExifData, FromExifData};
use crate::recipe::WBShift;
use log::trace;
use once_cell::sync::Lazy;
use regex::Regex;

impl FromExifData for WBShift {
    fn from_exif(data: &[ExifData]) -> Option<Self> {
        let exif = data.find("WhiteBalanceFineTune")?;

        trace!("WBShift::from_exif: {:?}", exif);

        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?i:(red ?[+\-0-9]+), ?(blue ?[+\-0-9]+))").unwrap());

        let captures = RE.captures(exif.value())?;

        trace!("WBShift Captures: {:?}", captures);

        let red = captures.get(1)?;
        let blue = captures.get(2)?;

        let red = red.as_str().to_lowercase().replace("red ", "");
        let blue = blue.as_str().to_lowercase().replace("blue ", "");

        let red: i32 = if let Ok(r) = red.parse::<i32>() {
            r
        } else {
            return None;
        };

        let blue: i32 = if let Ok(b) = blue.parse::<i32>() {
            b
        } else {
            return None;
        };

        let red = red / 10 / 2;
        let blue = blue / 10 / 2;

        trace!("red {:?}", red);
        trace!("blue {:?}", blue);

        Some(WBShift { red, blue })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_white_balance_shift() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalanceFineTune", "red 40, blue -60")];
        assert_eq!(
            WBShift::from_exif(&exif),
            Some(WBShift { red: 2, blue: -3 })
        );

        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalanceFineTune", "red -80, blue 20")];
        assert_eq!(
            WBShift::from_exif(&exif),
            Some(WBShift { red: -4, blue: 1 })
        );
    }

    #[test]
    fn it_should_should_not_parse_when_bad_number() {
        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalanceFineTune", "red 40.5, blue -60")];
        assert_eq!(WBShift::from_exif(&exif), None);

        let exif: Vec<ExifData> = vec![ExifData::new("WhiteBalanceFineTune", "red 20, blue hello")];
        assert_eq!(WBShift::from_exif(&exif), None);
    }

    #[test]
    fn it_should_should_not_parse_when_not_found() {
        let exif: Vec<ExifData> = vec![ExifData::new("Foo", "red 40, blue -60")];
        assert_eq!(WBShift::from_exif(&exif), None);
    }
}
