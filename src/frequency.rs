#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::ParsingError;

#[derive(Debug, Copy, Default, Clone, PartialEq, PartialOrd, Hash, Ord, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Frequency {
    /// DORIS #1 frequency
    #[default]
    DORIS1,

    /// DORIS #2 frequency
    DORIS2,
}

impl From<u8> for Frequency {
    fn from(val: u8) -> Self {
        match val {
            2 => Self::DORIS2,
            _ => Self::DORIS1,
        }
    }
}

impl std::str::FromStr for Frequency {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq("1") {
            return Ok(Self::DORIS1);
        } else if s.eq("2") {
            return Ok(Self::DORIS2);
        } else {
            return Err(ParsingError::Frequency);
        }
    }
}

impl std::fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DORIS1 => write!(f, "{}", '1'),
            Self::DORIS2 => write!(f, "{}", '2'),
        }
    }
}

impl Frequency {
    /// Returns frequency value in Hertz
    pub fn frequency_hz(&self) -> f64 {
        match self {
            Self::DORIS1 => 1.0,
            Self::DORIS2 => 2.0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Frequency;
    use std::str::FromStr;

    #[test]
    fn frequency_parsing() {
        for (value, expected) in [("1", Frequency::DORIS1), ("2", Frequency::DORIS2)] {
            let freq = Frequency::from_str(value).unwrap_or_else(|e| {
                panic!("failed to parse frequency from \"{}\": {}", value, e);
            });

            assert_eq!(freq, expected, "wrong value for {}", value);
        }
    }
}
