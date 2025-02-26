pub trait AddSign {
    fn add_sign(&self) -> String;
}

impl AddSign for i64 {
    fn add_sign(&self) -> String {
        match self {
            v if v > &0 => format!("+{}", v),
            v if v <= &0 => format!("{}", v),
            _ => unreachable!(),
        }
    }
}

impl AddSign for i32 {
    fn add_sign(&self) -> String {
        match self {
            v if v > &0 => format!("+{}", v),
            v if v <= &0 => format!("{}", v),
            _ => unreachable!(),
        }
    }
}

impl AddSign for f64 {
    fn add_sign(&self) -> String {
        match self {
            v if v > &0.0 => format!("+{}", v),
            v if v <= &0.0 => format!("{}", v),
            _ => unreachable!(),
        }
    }
}
