use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Grade {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Display for Grade {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Grade::A => write!(f, "A"),
            Grade::B => write!(f, "B"),
            Grade::C => write!(f, "C"),
            Grade::D => write!(f, "D"),
            Grade::E => write!(f, "E"),
            Grade::F => write!(f, "F"),
        }
    }
}

impl TryFrom<i32> for Grade {
    type Error = ();

    fn try_from(input: i32) -> Result<Self, Self::Error> {
        let value = match input {
            0 => Grade::A,
            1 => Grade::B,
            2 => Grade::C,
            3 => Grade::D,
            4 => Grade::E,
            5 => Grade::F,
            _ => return Err(())
        };
        Ok(value)
    }
}

impl FromStr for Grade {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let value = match input {
            "A" => Grade::A,
            "B" => Grade::B,
            "C" => Grade::C,
            "D" => Grade::D,
            "E" => Grade::E,
            "F" => Grade::F,
            _ => return Err(()),
        };

        Ok(value)
    }
}