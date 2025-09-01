#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::ParsingError;

/// [SNR] (Signal to Noise Ratio) for all frequency dependent measurements.
#[derive(Default, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SNR {
    /// SNR ~= 0 dB/Hz
    DbHz0,

    /// SNR < 12 dB/Hz
    DbHz12,

    /// 12 dB/Hz <= SNR < 17 dB/Hz
    DbHz12_17,

    /// 18 dB/Hz <= SNR < 23 dB/Hz
    DbHz18_23,

    /// 24 dB/Hz <= SNR < 29 dB/Hz
    #[default]
    DbHz24_29,

    /// 30 dB/Hz <= SNR < 35 dB/Hz
    DbHz30_35,

    /// 36 dB/Hz <= SNR < 41 dB/Hz
    DbHz36_41,

    /// 42 dB/Hz <= SNR < 47 dB/Hz
    DbHz42_47,

    /// 48 dB/Hz <= SNR < 53 dB/Hz
    DbHz48_53,

    /// SNR >= 54 dB/Hz
    DbHz54,
}

impl std::fmt::LowerHex for SNR {
    /// Prints [SNR] as per DORIS-RINEX files
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DbHz0 => f.write_str("0"),
            Self::DbHz12 => f.write_str("1"),
            Self::DbHz12_17 => f.write_str("2"),
            Self::DbHz18_23 => f.write_str("3"),
            Self::DbHz24_29 => f.write_str("4"),
            Self::DbHz30_35 => f.write_str("5"),
            Self::DbHz36_41 => f.write_str("6"),
            Self::DbHz42_47 => f.write_str("7"),
            Self::DbHz48_53 => f.write_str("8"),
            Self::DbHz54 => f.write_str("9"),
        }
    }
}

impl std::fmt::Display for SNR {
    /// Prints [SNR] in verbose manner
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DbHz0 => f.write_str("<< 12 dB"),
            Self::DbHz12 => f.write_str("< 12 dB"),
            Self::DbHz12_17 => f.write_str("[12, 17[ dB"),
            Self::DbHz18_23 => f.write_str("[18, 23[ dB"),
            Self::DbHz24_29 => f.write_str("[24, 29[ dB"),
            Self::DbHz30_35 => f.write_str("[30, 35[ dB"),
            Self::DbHz36_41 => f.write_str("[36, 41[ dB"),
            Self::DbHz42_47 => f.write_str("[42, 47[ dB"),
            Self::DbHz48_53 => f.write_str("[48, 53[ dB"),
            Self::DbHz54 => f.write_str("> 54 dB"),
        }
    }
}

impl std::str::FromStr for SNR {
    type Err = ParsingError;

    /// Parses [SNR] from standard DORIS-RINEX value.
    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match code.trim() {
            "0" => Ok(Self::DbHz0),
            "1" => Ok(Self::DbHz12),
            "2" => Ok(Self::DbHz12_17),
            "3" => Ok(Self::DbHz18_23),
            "4" => Ok(Self::DbHz24_29),
            "5" => Ok(Self::DbHz30_35),
            "6" => Ok(Self::DbHz36_41),
            "7" => Ok(Self::DbHz42_47),
            "8" => Ok(Self::DbHz48_53),
            "9" => Ok(Self::DbHz54),
            "bad" => Ok(Self::DbHz18_23),
            "weak" => Ok(Self::DbHz24_29),
            "strong" => Ok(Self::DbHz30_35),
            "excellent" => Ok(Self::DbHz48_53),
            _ => Err(ParsingError::SNR),
        }
    }
}

impl From<f64> for SNR {
    fn from(f_db: f64) -> Self {
        if f_db < 12.0 {
            Self::DbHz12
        } else if f_db <= 17.0 {
            Self::DbHz12_17
        } else if f_db <= 23.0 {
            Self::DbHz18_23
        } else if f_db <= 29.0 {
            Self::DbHz24_29
        } else if f_db <= 35.0 {
            Self::DbHz30_35
        } else if f_db <= 41.0 {
            Self::DbHz36_41
        } else if f_db <= 47.0 {
            Self::DbHz42_47
        } else if f_db <= 53.0 {
            Self::DbHz48_53
        } else {
            Self::DbHz54
        }
    }
}

impl From<SNR> for f64 {
    fn from(val: SNR) -> Self {
        match val {
            SNR::DbHz0 => 0.0_f64,
            SNR::DbHz12 => 12.0_f64,
            SNR::DbHz12_17 => 17.0_f64,
            SNR::DbHz18_23 => 23.0_f64,
            SNR::DbHz24_29 => 29.0_f64,
            SNR::DbHz30_35 => 35.0_f64,
            SNR::DbHz36_41 => 41.0_f64,
            SNR::DbHz42_47 => 47.0_f64,
            SNR::DbHz48_53 => 53.0_f64,
            SNR::DbHz54 => 54.0_f64,
        }
    }
}

impl From<u8> for SNR {
    fn from(u: u8) -> Self {
        match u {
            1 => Self::DbHz12,
            2 => Self::DbHz12_17,
            3 => Self::DbHz18_23,
            4 => Self::DbHz24_29,
            5 => Self::DbHz30_35,
            6 => Self::DbHz36_41,
            7 => Self::DbHz42_47,
            8 => Self::DbHz48_53,
            9 => Self::DbHz54,
            _ => Self::DbHz0,
        }
    }
}

impl SNR {
    /// Returns true if [SNR] is bad signal level.
    pub fn bad(self) -> bool {
        self <= Self::DbHz18_23
    }

    /// Returns true if [SNR] describes a weak signal level.
    pub fn weak(self) -> bool {
        self < Self::DbHz30_35
    }

    /// Returns true if [SNR] describes a strong signal level.
    pub fn strong(self) -> bool {
        self >= Self::DbHz30_35
    }

    /// Returns true if [SNR] describes an exellent signal level.
    pub fn excellent(self) -> bool {
        self > Self::DbHz42_47
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn snr_parsing() {
        for (value, expected) in [("0", SNR::DbHz0), ("8", SNR::DbHz48_53), ("9", SNR::DbHz54)] {
            let parsed = SNR::from_str(value).unwrap_or_else(|e| {
                panic!("Failed to parse SNR from \"{}\"", value);
            });

            let formatted = format!("{:x}", parsed);

            assert_eq!(formatted, value);
        }

        assert!(SNR::DbHz0.bad());
        assert!(SNR::DbHz12.weak());
        assert!(SNR::DbHz30_35.strong());
        assert!(SNR::DbHz54.excellent());
    }
}
