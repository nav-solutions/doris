#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Hash, Ord, Eq)]
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

impl std::fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DORIS1 => write!(f, '1'),
            Self::DORIS2 => write!(f, '2'),
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
            let freq = Frequency::from_str(value)
                .unwrap_or_else(|e| {
                    panic!("failed to parse frequency from \"{}\"", value);
                });

            assert_eq!(freq, expected, "wrong value for {}", value);
        }
    }
}
