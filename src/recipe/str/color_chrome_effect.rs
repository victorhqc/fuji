use crate::recipe::ColorChromeEffect;
use std::fmt::{Display, Formatter};

impl Display for ColorChromeEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.strength.to_string())
    }
}
