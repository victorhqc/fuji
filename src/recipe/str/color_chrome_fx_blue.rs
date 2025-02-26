use crate::recipe::ColorChromeEffectFxBlue;
use std::fmt::{Display, Formatter};

impl Display for ColorChromeEffectFxBlue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.strength.to_string())
    }
}
