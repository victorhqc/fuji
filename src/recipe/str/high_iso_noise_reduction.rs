use crate::recipe::HighISONoiseReduction;
use crate::utils::str::AddSign;
use std::fmt::{Display, Formatter};

impl Display for HighISONoiseReduction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value.add_sign())
    }
}
