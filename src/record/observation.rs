use std::str::FromStr;

#[cfg(doc)]
use crate::prelude::Observable;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{error::ParsingError, prelude::SNR};

/// Signal [Observation]
#[derive(Copy, Default, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Observation {
    /// [SNR] for all frequency measurements
    pub snr: Option<SNR>,

    // /// Phase lock [Flag] for phase measurements specifically.
    // pub phase_flag: Option<Flag>,
    /// Measured value, unit is [Observable] dependent.
    pub value: f64,
}

impl Observation {
    /// Define DORIS [Observation] with [SNR] value
    pub fn with_snr(mut self, snr: SNR) -> Self {
        self.snr = Some(snr);
        self
    }

    // /// Defines DORIS phase measurement with associated [Flag]
    // pub fn with_phase_flag(mut self, flag: Flag) -> Self {
    //     self.phase_flag = Some(flag);
    //     self
    // }

    /// Defines new DORIS measurement with desired value
    pub fn with_value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn default_flag() {
    //     assert_eq!(Flag::default(), Flag::Ok);
    // }

    // #[test]
    // fn parsing() {
    //     for (flag, expected) in [("0", Flag::Ok), ("1", Flag::PowerFailure)] {
    //         let parsed = Flag::from_str(flag).unwrap();

    //         assert_eq!(parsed, expected);

    //         let formatted = parsed.to_string();

    //         assert_eq!(formatted, flag);
    //     }
    // }
}
