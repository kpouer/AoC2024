use std::fmt::Display;

#[derive(PartialEq)]
pub enum Direction {
    Increasing,
    Decreasing,
    Indeterminate,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Increasing => write!(f, "Increasing"),
            Direction::Decreasing => write!(f, "Decreasing"),
            Direction::Indeterminate => write!(f, "Indeterminate"),
        }
    }
}

impl From<bool> for Direction {
    fn from(value: bool) -> Self {
        if value {
            Direction::Increasing
        } else {
            Direction::Decreasing
        }
    }
}
