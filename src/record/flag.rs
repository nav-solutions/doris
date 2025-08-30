use std::str::FromStr;

use crate::error::ParsingError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Flag attached to Phase locked observations,
/// describing the lock status.
#[derive(Copy, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Flag {
    /// Observation is sane
    #[default]
    Ok,

    /// Power failure since previous epoch
    PowerFailure,

    /// Antenna is being moved at current epoch
    AntennaBeingMoved,

    /// Site has changed, received has moved since last epoch
    NewSiteOccupation,

    /// New information to come after this epoch
    HeaderInformationFollows,

    /// External event - significant event at this epoch.
    ExternalEvent,

    /// Cycle slip at this epoch.
    CycleSlip,
}

impl FromStr for Flag {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Ok),
            "1" => Ok(Self::PowerFailure),
            "2" => Ok(Self::AntennaBeingMoved),
            "3" => Ok(Self::NewSiteOccupation),
            "4" => Ok(Self::HeaderInformationFollows),
            "5" => Ok(Self::ExternalEvent),
            "6" => Ok(Self::CycleSlip),
            _ => Err(ParsingError::ObservationFlag),
        }
    }
}

impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ok => "0".fmt(f),
            Self::PowerFailure => "1".fmt(f),
            Self::AntennaBeingMoved => "2".fmt(f),
            Self::NewSiteOccupation => "3".fmt(f),
            Self::HeaderInformationFollows => "4".fmt(f),
            Self::ExternalEvent => "5".fmt(f),
            Self::CycleSlip => "6".fmt(f),
        }
    }
}
