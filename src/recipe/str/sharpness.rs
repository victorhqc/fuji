use crate::recipe::Sharpness;
use crate::utils::str::AddSign;
use std::fmt::{Display, Formatter};

impl Display for Sharpness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value.add_sign())
    }
}
