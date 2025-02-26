use crate::recipe::ToneCurve;
use std::fmt::{Display, Formatter};

impl Display for ToneCurve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "H{} S{}", self.highlights, self.shadows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_to_string() {
        let result = ToneCurve {
            highlights: 1.5,
            shadows: 2.0,
        }
        .to_string();

        assert_eq!(result, "H1.5 S2");
    }

    #[test]
    fn it_handles_negative_numbers() {
        let result = ToneCurve {
            highlights: -2.0,
            shadows: -3.5,
        }
        .to_string();

        assert_eq!(result, "H-2 S-3.5");
    }
}
