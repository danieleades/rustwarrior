use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "u8", into = "u8")]
pub enum Priority {
    One,
    Two,
    Three,
    Four,
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(u8::from(*other).cmp(&u8::from(*self)))
    }
}

impl From<Priority> for u8 {
    fn from(priority: Priority) -> Self {
        match priority {
            Priority::One => 1,
            Priority::Two => 2,
            Priority::Three => 3,
            Priority::Four => 4,
        }
    }
}

impl TryFrom<u8> for Priority {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            x => Err(Error(x)),
        }
    }
}

#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
#[error("invalid priority value '{0}'. Expected 1-4")]
pub struct Error(u8);

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::{Error, Priority};

    #[test_case(0 => Err(Error(0)); "0")]
    #[test_case(1 => Ok(Priority::One); "1")]
    #[test_case(2 => Ok(Priority::Two); "2")]
    #[test_case(3 => Ok(Priority::Three); "3")]
    #[test_case(4 => Ok(Priority::Four); "4")]
    #[test_case(5 => Err(Error(5)); "5")]
    fn parse(input: u8) -> Result<Priority, Error> {
        Priority::try_from(input)
    }

    #[test]
    fn ord() {
        assert!(Priority::One > Priority::Two);
        assert!(Priority::Two > Priority::Three);
        assert!(Priority::Three > Priority::Four);
    }
}
