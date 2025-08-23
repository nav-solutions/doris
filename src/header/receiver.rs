//! Receiver and antenna
use crate::{
    fmt_rinex,
    prelude::{FormattingError, COSPAR, SV},
};

use std::{
    io::{BufWriter, Write},
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// GNSS receiver description
#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Receiver {
    /// Receiver (hardware) model
    pub model: String,

    /// Receiver (hardware) identification info
    pub serial_number: String,

    /// Receiver embedded software info
    pub firmware: String,
}

impl Receiver {
    /// Formats [Receiver] into [BufWriter]
    pub(crate) fn format<W: Write>(&self, w: &mut BufWriter<W>) -> Result<(), FormattingError> {
        writeln!(
            w,
            "{}",
            fmt_rinex(
                &format!("{:<20}{:<20}{}", self.sn, self.model, self.firmware),
                "REC # / TYPE / VERS"
            )
        )?;
        Ok(())
    }

    pub fn with_model(&self, model: &str) -> Self {
        let mut s = self.clone();
        s.model = model.to_string();
        s
    }

    pub fn with_serial_number(&self, sn: &str) -> Self {
        let mut s = self.clone();
        s.sn = sn.to_string();
        s
    }

    pub fn with_firmware(&self, firmware: &str) -> Self {
        let mut s = self.clone();
        s.firmware = firmware.to_string();
        s
    }
}

impl FromStr for Receiver {
    type Err = std::io::Error;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, rem) = line.split_at(20);
        let (make, rem) = rem.split_at(20);
        let (version, _) = rem.split_at(20);
        Ok(Receiver {
            sn: id.trim().to_string(),
            model: make.trim().to_string(),
            firmware: version.trim().to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn receiver_parsing() {
        let content = "2090088             LEICA GR50          4.51                ";
        let rcvr = Receiver::from_str(content).unwrap();

        assert_eq!(rcvr.model, "LEICA GR50");
        assert_eq!(rcvr.sn, "2090088");
        assert_eq!(rcvr.firmware, "4.51");
    }
}
