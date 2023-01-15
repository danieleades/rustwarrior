use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

/// The task priority. [`Priority::One`] (1) is the highest.
///
/// When compared, priorities are sorted lowest to highest. This means that
/// [`Priority::Three`] is considered the *lowest* value, and [`Priority::One`]
/// the the highest value.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "u8", into = "u8")]
pub enum Priority {
    /// P1 priority
    One,

    /// P2 priority
    Two,

    /// P3 priority
    Three,
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
        }
    }
}

impl TryFrom<u8> for Priority {
    type Error = Error<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            x => Err(Error(x)),
        }
    }
}

impl FromStr for Priority {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let int: u8 = s.parse().map_err(|_| Error(s.to_string()))?;
        Self::try_from(int).map_err(|_| Error(s.to_string()))
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", u8::from(*self))
    }
}

#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
#[error("invalid priority value '{0}'. Expected 1-4")]
pub struct Error<T>(T);

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::{Error, Priority};

    #[test_case(0 => Err(Error(0)); "0")]
    #[test_case(1 => Ok(Priority::One); "1")]
    #[test_case(2 => Ok(Priority::Two); "2")]
    #[test_case(3 => Ok(Priority::Three); "3")]
    #[test_case(5 => Err(Error(5)); "5")]
    fn parse(input: u8) -> Result<Priority, Error<u8>> {
        Priority::try_from(input)
    }

    #[test]
    fn ord() {
        assert!(Priority::One > Priority::Two);
        assert!(Priority::Two > Priority::Three);
    }
}
