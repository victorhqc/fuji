use crate::recipe::{
    str::{Error, ParseKey},
    GrainEffect, GrainSize, GrainStrength,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

impl GrainEffect {
    pub fn grain_strength_to_string(&self) -> Option<String> {
        match self {
            GrainEffect::Off => None,
            GrainEffect::OnlyStrength { strength } => Some(strength.to_string()),
            GrainEffect::StrengthAndSize { strength, size: _ } => Some(strength.to_string()),
        }
    }

    pub fn grain_size_to_string(&self) -> Option<String> {
        match self {
            GrainEffect::Off => None,
            GrainEffect::OnlyStrength { strength: _ } => None,
            GrainEffect::StrengthAndSize { strength: _, size } => Some(size.to_string()),
        }
    }
}

impl FromStr for GrainEffect {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(Off|(Strong|Weak)?(,\s)?(Small|Large)?)").unwrap());
        let caps = RE.captures(s).unwrap();

        if caps[1].is_empty() {
            return Err(Error::Parse {
                key: ParseKey::GrainEffect,
                reason: format!("invalid GrainEffect: '{}'", s),
            });
        }

        if &caps[1] == "Off" {
            return Ok(GrainEffect::Off);
        }

        if let (Some(str), None, None) = (caps.get(1), caps.get(3), caps.get(4)) {
            let strength = GrainStrength::from_str(str.as_str()).unwrap();
            return Ok(GrainEffect::OnlyStrength { strength });
        }

        if let (Some(str), Some(size)) = (caps.get(2), caps.get(4)) {
            let strength = GrainStrength::from_str(str.as_str()).unwrap();
            let size = GrainSize::from_str(size.as_str()).unwrap();

            return Ok(GrainEffect::StrengthAndSize { strength, size });
        }

        Err(Error::Parse {
            key: ParseKey::GrainEffect,
            reason: format!("invalid GrainEffect: {}", s),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::{
        str::{Error, ParseKey},
        GrainSize, GrainStrength,
    };

    #[test]
    fn it_parses_off() {
        let off = GrainEffect::Off.to_string();

        assert_eq!(&off, "Off");
        assert_eq!(GrainEffect::from_str(&off).unwrap(), GrainEffect::Off);
    }

    #[test]
    fn it_parses_strength_only() {
        let strong = GrainEffect::OnlyStrength {
            strength: GrainStrength::Strong,
        }
        .to_string();
        let weak = GrainEffect::OnlyStrength {
            strength: GrainStrength::Weak,
        }
        .to_string();

        assert_eq!(&strong, "Strong");
        assert_eq!(
            GrainEffect::from_str(&strong).unwrap(),
            GrainEffect::OnlyStrength {
                strength: GrainStrength::Strong,
            }
        );

        assert_eq!(&weak, "Weak");
        assert_eq!(
            GrainEffect::from_str(&weak).unwrap(),
            GrainEffect::OnlyStrength {
                strength: GrainStrength::Weak,
            }
        );
    }

    #[test]
    fn it_parses_strength_and_size() {
        let strong_small = GrainEffect::StrengthAndSize {
            strength: GrainStrength::Strong,
            size: GrainSize::Small,
        }
        .to_string();
        let strong_large = GrainEffect::StrengthAndSize {
            strength: GrainStrength::Strong,
            size: GrainSize::Large,
        }
        .to_string();
        let weak_small = GrainEffect::StrengthAndSize {
            strength: GrainStrength::Weak,
            size: GrainSize::Small,
        }
        .to_string();
        let weak_large = GrainEffect::StrengthAndSize {
            strength: GrainStrength::Weak,
            size: GrainSize::Large,
        }
        .to_string();

        assert_eq!(&strong_small, "Strong, Small");
        assert_eq!(
            GrainEffect::from_str(&strong_small).unwrap(),
            GrainEffect::StrengthAndSize {
                strength: GrainStrength::Strong,
                size: GrainSize::Small,
            }
        );

        assert_eq!(&strong_large, "Strong, Large");
        assert_eq!(
            GrainEffect::from_str(&strong_large).unwrap(),
            GrainEffect::StrengthAndSize {
                strength: GrainStrength::Strong,
                size: GrainSize::Large,
            }
        );

        assert_eq!(&weak_small, "Weak, Small");
        assert_eq!(
            GrainEffect::from_str(&weak_small).unwrap(),
            GrainEffect::StrengthAndSize {
                strength: GrainStrength::Weak,
                size: GrainSize::Small,
            }
        );

        assert_eq!(&weak_large, "Weak, Large");
        assert_eq!(
            GrainEffect::from_str(&weak_large).unwrap(),
            GrainEffect::StrengthAndSize {
                strength: GrainStrength::Weak,
                size: GrainSize::Large,
            }
        );
    }

    #[test]
    fn it_fails_on_empty_grain_effect() {
        let result = GrainEffect::from_str("");
        let expected = Error::Parse {
            key: ParseKey::GrainEffect,
            reason: "invalid GrainEffect: ''".to_string(),
        };

        assert_eq!(result, Err(expected));
    }

    #[test]
    fn it_fails_on_wrong_grain_effect() {
        let result = GrainEffect::from_str("whatever man");
        let expected = Error::Parse {
            key: ParseKey::GrainEffect,
            reason: "invalid GrainEffect: 'whatever man'".to_string(),
        };

        assert_eq!(result, Err(expected));
    }
}
