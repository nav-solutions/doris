use crate::prelude::{Error, ParsingError};

#[cfg(feature = "serde", derive(Serialize, Deserialize))]
use serde::{Serialize, Deserialize};

/// [Observable] describes both frequency and physics.
/// For example, [Observable::PhaseRange] and [Observable::Power] are two different physics.
/// DORIS files also provides information sampled at the ground station level for high
/// precision models (like pressure and moisture rate).
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Observable {
    /// Carrier phase observation (in meters, not cycles)
    PhaseRange(Frequency),
    
    /// Decoded Pseudo range (in meters)
    PseudoRange(Frequency),

    /// Received signal power (in dBm)
    Power(Frequency),

    /// Pressure at ground station level, in hPa.
    Pressure,

    /// Dry temperature at ground station level, in celcius degrees.
    Temperature,

    /// Moisture rate at ground station level, as saturation percentage.
    HumidityRate,

    /// DORIS frequencies ratio (dimensionless), image of the frequency drift
    FrequencyRatio,
}

impl Default for Observable {
    fn default() -> Self {
        Self::PhaseRange(Default::default())
    }
}

impl Observable {
    /// Returns true if both [Observable]s come from the same [Frequency]
    pub fn same_frequency(&self, rhs: &Observable) -> bool {
        match self {
            Self::PseudoRange(freq) |
            Self::Power(freq) |
            Self::PhaseRange(freq) => match rhs {
                Self::PseudoRange(rhs) 
                | Self::Power(rhs)
                | Self::PhaseRange(rhs) => {
                    rhs == freq
                },
            },
            _ => false,
        }
    }

    /// Returns true if Self and rhs describe the same physical observation.
    /// For example, both are phase observations.
    pub fn same_physics(&self, rhs: &Observable) -> bool {
        match self {
            Self::PhaseRange(_) => matches!(rhs, Self::PhaseRange(_)),
            Self::PseudoRange(_) => matches!(rhs, Self::PseudoRange(_)),
            Self::Power(_) => matches!(rhs, Self::Power(_)),
            Self::Pressure => matches!(rhs, Self::Pressure),
            Self::Temperature => matches!(rhs, Self::Temperature),
            Self::HumidityRate => matches!(rhs, Self::HumidityRate),
        }
    }

    /// Returns true if this a [Observable::PhaseRange] measurement
    pub fn is_phase_range_observable(&self) -> bool {
        matches!(self, Self::PhaseRange(_))
    }

    /// Returns true if this [Observable] is an [Observable::PhaseRange] measurement
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
            Self::PseudoRange(freq) => write!(f, "L{}", freq),
            Self::PhaseRange(freq) => write!(f, "C{}", freq),
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
                let frequency = Frequency::from_str(content[1..])?;
                if content.starts_with('L') {
                    Ok(Self::PhaseRange(frequency))
                } else if content.starts_with('C') {
                    Ok(Self::PseudoRange(frequency))
                } else if content.starts_with('W') {
                    Ok(Self::Power(frequency))
                } else {
                    Err(ParsingError::ObservableParsing)
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::prelude::{Observable, Frequency};
    
    #[test]
    fn test_default_observable() {
        let default = Observable::default();
        assert_eq!(Observable::default(), Observable::PhaseRange(Frequency::DORIS1));

        let formatted = default.to_string();
        let parsed = Observable::from_str(formatted)
            .unwrap_or_else(|e| {
                panic!("Failed to parse observable from \"{}\"", formatted);
            });
    
        assert_eq!(parsed, default);
    }

    #[test]
    fn test_physics() {
        assert!(Observable::from_str("L1")
            .unwrap()
            .is_phase_range_observable());
        assert!(Observable::from_str("L2")
            .unwrap()
            .is_phase_range_observable());
        assert!(Observable::from_str("L6X")
            .unwrap()
            .is_phase_range_observable());
        assert!(Observable::from_str("C1")
            .unwrap()
            .is_pseudo_range_observable());
        assert!(Observable::from_str("C2")
            .unwrap()
            .is_pseudo_range_observable());
        assert!(Observable::from_str("C6X")
            .unwrap()
            .is_pseudo_range_observable());
        assert!(Observable::from_str("D1").unwrap().is_doppler_observable());
        assert!(Observable::from_str("D2").unwrap().is_doppler_observable());
        assert!(Observable::from_str("D6X").unwrap().is_doppler_observable());
        assert!(Observable::from_str("S1").unwrap().is_ssi_observable());
        assert!(Observable::from_str("S2").unwrap().is_ssi_observable());
        assert!(Observable::from_str("S1P").unwrap().is_ssi_observable());
        assert!(Observable::from_str("S1W").unwrap().is_ssi_observable());
    }
    #[test]
    fn test_observable() {
        assert_eq!(Observable::from_str("PR").unwrap(), Observable::Pressure);
        assert_eq!(Observable::from_str("pr").unwrap(), Observable::Pressure);
        assert_eq!(Observable::from_str("PR").unwrap().to_string(), "PR");

        assert_eq!(Observable::from_str("WS").unwrap(), Observable::WindSpeed);
        assert_eq!(Observable::from_str("ws").unwrap(), Observable::WindSpeed);
        assert_eq!(Observable::from_str("WS").unwrap().to_string(), "WS");

        assert!(Observable::from_str("Err").is_err());
        assert!(Observable::from_str("TODO").is_err());

        assert_eq!(
            Observable::from_str("L1").unwrap(),
            Observable::PhaseRange(String::from("L1"))
        );

        assert!(Observable::from_str("L1").unwrap().code().is_none());

        assert_eq!(
            Observable::from_str("L2").unwrap(),
            Observable::PhaseRange(String::from("L2"))
        );

        assert_eq!(
            Observable::from_str("L5").unwrap(),
            Observable::PhaseRange(String::from("L5"))
        );
        assert_eq!(
            Observable::from_str("L6Q").unwrap(),
            Observable::PhaseRange(String::from("L6Q"))
        );
        assert_eq!(
            Observable::from_str("L6Q").unwrap().code(),
            Some(String::from("6Q")),
        );

        assert_eq!(
            Observable::from_str("L1C").unwrap(),
            Observable::PhaseRange(String::from("L1C"))
        );
        assert_eq!(
            Observable::from_str("L1P").unwrap(),
            Observable::PhaseRange(String::from("L1P"))
        );
        assert_eq!(
            Observable::from_str("L8X").unwrap(),
            Observable::PhaseRange(String::from("L8X"))
        );

        assert_eq!(
            Observable::from_str("L1P").unwrap(),
            Observable::PhaseRange(String::from("L1P"))
        );

        assert_eq!(
            Observable::from_str("L8X").unwrap(),
            Observable::PhaseRange(String::from("L8X"))
        );

        assert_eq!(
            Observable::from_str("S7Q").unwrap(),
            Observable::SSI(String::from("S7Q")),
        );

        assert_eq!(
            Observable::PseudoRange("S7Q".to_string()).to_string(),
            "S7Q",
        );

        assert_eq!(Observable::Doppler("D7Q".to_string()).to_string(), "D7Q",);

        assert_eq!(Observable::Doppler("C7X".to_string()).to_string(), "C7X",);
    }

    #[test]
    fn test_same_physics() {
        assert!(Observable::Temperature.same_physics(&Observable::Temperature));
        assert!(!Observable::Pressure.same_physics(&Observable::Temperature));

        let dop_l1 = Observable::Doppler("L1".to_string());
        let dop_l1c = Observable::Doppler("L1C".to_string());
        let dop_l2 = Observable::Doppler("L2".to_string());
        let dop_l2w = Observable::Doppler("L2W".to_string());

        let pr_l1 = Observable::PseudoRange("L1".to_string());
        let pr_l1c = Observable::PseudoRange("L1C".to_string());
        let pr_l2 = Observable::PseudoRange("L2".to_string());
        let pr_l2w = Observable::PseudoRange("L2W".to_string());

        assert!(dop_l1.same_physics(&dop_l1));
        assert!(dop_l1c.same_physics(&dop_l1));
        assert!(dop_l1c.same_physics(&dop_l2));
        assert!(dop_l1c.same_physics(&dop_l2w));
        assert!(!dop_l1.same_physics(&pr_l1));
        assert!(!dop_l1.same_physics(&pr_l1c));
        assert!(!dop_l1.same_physics(&pr_l2));
        assert!(!dop_l1.same_physics(&pr_l2w));

        assert!(pr_l1.same_physics(&pr_l1));
        assert!(pr_l1.same_physics(&pr_l1c));
        assert!(pr_l1.same_physics(&pr_l2));
        assert!(pr_l1.same_physics(&pr_l2w));
    }
}
