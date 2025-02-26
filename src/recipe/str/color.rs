use crate::recipe::Color;
use crate::utils::str::AddSign;
use std::fmt::{Display, Formatter};

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value.add_sign())
    }
}
