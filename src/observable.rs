#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{error::ParsingError, frequency::Frequency};

/// [Observable] describes both frequency and physics.
/// For example, [Observable::UnambiguousPhaseRange] and [Observable::Power] are two different physics.
/// DORIS files also provides information sampled at the ground station level for high
/// precision models (like pressure and moisture rate).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Ord, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Observable {
    /// Decoded Pseudo range (in meters).  
    /// All frequency dependent DORIS measurements (=signal measurements) were
    /// performed in space.
    PseudoRange(Frequency),

    /// Unambiguous carrier phase observation (in meters, not cycles).  
    /// Unlike standard RINEX observations, ambiguities have been resolved
    /// prior publishing a DORIS file.
    /// All frequency dependent DORIS measurements (=signal measurements) were
    /// performed in space.
    UnambiguousPhaseRange(Frequency),

    /// Received signal power, in \[dBm\].  
    /// All frequency dependent DORIS measurements (=signal measurements) were
    /// performed in space.  
    Power(Frequency),

    /// Pressure at ground station level at epoch of spaceborn observation, in \[hPa\].
    Pressure,

    /// Dry temperature at ground station level at epoch of spaceborn observation, in celcius degrees.
    Temperature,

    /// Moisture rate at ground station level at epoch of spaceborn observation, as saturation percentage.
    HumidityRate,

    /// f1 / f2 frequency ratio (dimensionless)
    FrequencyRatio,
}

impl Default for Observable {
    fn default() -> Self {
        Self::PseudoRange(Default::default())
    }
}

impl Observable {
    /// Returns true if both [Observable]s come from the same [Frequency]
    pub fn same_frequency(&self, rhs: &Observable) -> bool {
        match self {
            Self::PseudoRange(freq) | Self::Power(freq) | Self::UnambiguousPhaseRange(freq) => {
                match rhs {
                    Self::PseudoRange(rhs)
                    | Self::Power(rhs)
                    | Self::UnambiguousPhaseRange(rhs) => rhs == freq,
                    _ => false,
                }
            },
            _ => false,
        }
    }

    /// Returns true if Self and rhs describe the same physical observation.
    /// For example, both are phase observations.
    pub fn same_physics(&self, rhs: &Observable) -> bool {
        match self {
            Self::UnambiguousPhaseRange(_) => matches!(rhs, Self::UnambiguousPhaseRange(_)),
            Self::PseudoRange(_) => matches!(rhs, Self::PseudoRange(_)),
            Self::Power(_) => matches!(rhs, Self::Power(_)),
            Self::Pressure => matches!(rhs, Self::Pressure),
            Self::Temperature => matches!(rhs, Self::Temperature),
            Self::HumidityRate => matches!(rhs, Self::HumidityRate),
            Self::FrequencyRatio => matches!(rhs, Self::FrequencyRatio),
        }
    }

    /// Returns true if this a [Observable::UnambiguousPhaseRange] measurement
    pub fn is_phase_range_observable(&self) -> bool {
        matches!(self, Self::UnambiguousPhaseRange(_))
    }

    /// Returns true if this [Observable] is an [Observable::UnambiguousPhaseRange] measurement
    pub fn is_pseudo_range_observable(&self) -> bool {
        matches!(self, Self::PseudoRange(_))
    }

    pub fn is_power_observable(&self) -> bool {
        matches!(self, Self::Power(_))
    }
}

impl std::fmt::Display for Observable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pressure => write!(f, "Pressure"),
            Self::Temperature => write!(f, "Temperature"),
            Self::HumidityRate => write!(f, "Moisture rate"),
            Self::FrequencyRatio => write!(f, "Frequency ratio"),
            Self::PseudoRange(freq) => write!(f, "C{}", freq),
            Self::UnambiguousPhaseRange(freq) => write!(f, "L{}", freq),
            Self::Power(freq) => write!(f, "W{}", freq),
        }
    }
}

impl std::fmt::LowerHex for Observable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pressure => write!(f, "P"),
            Self::Temperature => write!(f, "T"),
            Self::HumidityRate => write!(f, "H"),
            Self::FrequencyRatio => write!(f, "F"),
            Self::PseudoRange(freq) => write!(f, "C{}", freq),
            Self::UnambiguousPhaseRange(freq) => write!(f, "L{}", freq),
            Self::Power(freq) => write!(f, "W{}", freq),
        }
    }
}

impl std::str::FromStr for Observable {
    type Err = ParsingError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let content = content.to_uppercase();
        let content = content.trim();
        match content {
            "P" | "PRESSURE" => Ok(Self::Pressure),
            "T" | "TEMPERATURE" => Ok(Self::Temperature),
            "H" | "MOISTURE RATE" => Ok(Self::HumidityRate),
            "F" | "FREQUENCY RATIO" => Ok(Self::FrequencyRatio),
            _ => {
                let frequency = Frequency::from_str(&content[1..])?;
                if content.starts_with('L') {
                    Ok(Self::UnambiguousPhaseRange(frequency))
                } else if content.starts_with('C') {
                    Ok(Self::PseudoRange(frequency))
                } else if content.starts_with('W') {
                    Ok(Self::Power(frequency))
                } else {
                    Err(ParsingError::Observable)
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::{Frequency, Observable};
    use std::str::FromStr;

    #[test]
    fn test_default_observable() {
        let default = Observable::default();

        assert_eq!(default, Observable::PseudoRange(Frequency::DORIS1));

        let formatted = default.to_string();

        let parsed = Observable::from_str(&formatted).unwrap_or_else(|e| {
            panic!("Failed to parse observable from \"{}\"", formatted);
        });

        assert_eq!(parsed, default);
    }

    #[test]
    fn observable_parsing() {
        for (observable, expected, formatted) in [
            (
                "L1",
                Observable::UnambiguousPhaseRange(Frequency::DORIS1),
                "L1",
            ),
            (
                "L2",
                Observable::UnambiguousPhaseRange(Frequency::DORIS2),
                "L2",
            ),
            ("C1", Observable::PseudoRange(Frequency::DORIS1), "C1"),
            ("C2", Observable::PseudoRange(Frequency::DORIS2), "C2"),
            ("W1", Observable::Power(Frequency::DORIS1), "W1"),
            ("W2", Observable::Power(Frequency::DORIS2), "W2"),
            ("T", Observable::Temperature, "Temperature"),
            ("P", Observable::Pressure, "Pressure"),
            ("H", Observable::HumidityRate, "Moisture rate"),
        ] {
            let parsed = Observable::from_str(observable).unwrap_or_else(|e| {
                panic!("failed to parse observable from \"{}\": {}", observable, e);
            });

            assert_eq!(parsed, expected);
            assert_eq!(parsed.to_string(), formatted);
        }

        let l1 = Observable::UnambiguousPhaseRange(Frequency::DORIS1);
        let l2 = Observable::UnambiguousPhaseRange(Frequency::DORIS2);
        let c1 = Observable::PseudoRange(Frequency::DORIS1);
        let c2 = Observable::PseudoRange(Frequency::DORIS2);

        assert!(l1.same_physics(&l1));
        assert!(l1.same_physics(&l2));

        assert!(c1.same_physics(&c1));
        assert!(c1.same_physics(&c2));

        assert!(!l1.same_physics(&c1));
        assert!(!l1.same_physics(&c2));
    }
}
