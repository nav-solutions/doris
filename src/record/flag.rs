use crate::error::ParsingError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [EpochFlag] is attached to DORIS epochs,
/// describing sampling conditions and attached data.
#[derive(Copy, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EpochFlag {
    /// Epoch is OK (sane)
    #[default]
    OK,

    /// Power failure since previous epoch
    PowerFailure,

    /// Special event: antenna being moved since previous measurement
    AntennaBeingMoved,

    /// Special event: new site occupation (marks end of kinematic data)
    NewSiteEndofKinematics,

    /// Header information is to follow (not actual measurements)
    HeaderDataFollowing,

    /// External event (other)
    ExternalEvent,
}

impl std::str::FromStr for EpochFlag {
    type Err = ParsingError;

    /// Parses [EpochFlag] from standard values.    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::OK),
            "1" => Ok(Self::PowerFailure),
            "2" => Ok(Self::AntennaBeingMoved),
            "3" => Ok(Self::NewSiteEndofKinematics),
            "4" => Ok(Self::HeaderDataFollowing),
            "5" => Ok(Self::ExternalEvent),
            _ => Err(ParsingError::EpochFlag),
        }
    }
}

impl std::fmt::Display for EpochFlag {
    /// Formats [EpochFlag] according to standards.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OK => "0".fmt(f),
            Self::PowerFailure => "1".fmt(f),
            Self::AntennaBeingMoved => "2".fmt(f),
            Self::NewSiteEndofKinematics => "3".fmt(f),
            Self::HeaderDataFollowing => "4".fmt(f),
            Self::ExternalEvent => "5".fmt(f),
        }
    }
}
