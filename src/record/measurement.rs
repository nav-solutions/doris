#[cfg(doc)]
use crate::prelude::{GroundStation, TimeScale, DORIS};

use crate::{
    error::ParsingError,
    prelude::{Duration, Observable, Observation},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClockOffset {
    /// True if this [ClockOffset] is actually extrapolated
    /// and not actually measured.
    pub extrapolated: bool,

    /// Offset to [TimeScale::TAI] timescale, as [Duration]
    pub offset: Duration,
}

impl ClockOffset {
    /// Creates new [ClockOffset] from measured offset.
    pub fn from_measured_offset(offset: Duration) -> Self {
        Self {
            offset,
            extrapolated: false,
        }
    }

    /// Creates new [ClockOffset] from extrapolated offset
    /// (not actually measured).
    pub fn from_extrapolated_offset(offset: Duration) -> Self {
        Self {
            offset,
            extrapolated: true,
        }
    }
}

/// [MeasurementFlag] is attached to DORIS measurements,
/// describing sampling conditions.
#[derive(Copy, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MeasurementFlag {
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

impl std::str::FromStr for MeasurementFlag {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::OK),
            "1" => Ok(Self::PowerFailure),
            "2" => Ok(Self::AntennaBeingMoved),
            "3" => Ok(Self::NewSiteEndofKinematics),
            "4" => Ok(Self::HeaderDataFollowing),
            "5" => Ok(Self::ExternalEvent),
            _ => Err(ParsingError::MeasurementFlag),
        }
    }
}

impl std::fmt::Display for MeasurementFlag {
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
/// [DORIS] Measurements (also referred to as "Observations") of a [GroundStation]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Measurements {
    /// Measurement flag, describing sampling conditions
    pub flag: MeasurementFlag,

    /// Satellite (=measurement system) [ClockOffset].
    pub satellite_clock_offset: Option<ClockOffset>,

    /// Observations indexed [Observable]s, measurement unit varies.
    pub observations: HashMap<Observable, Observation>,
}

impl Measurements {
    /// Returns true if this set of [Measurements] is marked with [MeasurementFlag::OK].
    /// When this is not true, you should use the attached [MeasurementFlag] to inquire
    /// the event, and likely issue.
    pub fn is_ok(&self) -> bool {
        self.flag == MeasurementFlag::OK
    }

    /// Add a new observation to this set of [Measurements]  
    pub fn add_observation(&mut self, observable: Observable, observation: Observation) {
        self.observations.insert(observable, observation);
    }

    /// Updates this set of [Measurements] with a new observation
    pub fn with_observation(&self, observable: Observable, observation: Observation) -> Self {
        let mut s = self.clone();
        s.observations.insert(observable, observation);
        s
    }

    /// Returns a unique list of [Observable]s, defining all physics
    /// measured in this set of [Measurement]
    pub fn observables(&self) -> Box<dyn Iterator<Item = Observable> + '_> {
        Box::new(self.observations.keys().map(|obs| *obs).unique())
    }

    /// Copies and returns [Measurements] with updated [ClockOffset]
    pub fn with_satellite_clock_offset(&self, clock_offset: ClockOffset) -> Self {
        let mut s = self.clone();
        s.satellite_clock_offset = Some(clock_offset);
        s
    }

    /// Copies and updates the [MeasurementFlag]
    pub fn with_measurement_flag(&self, flag: MeasurementFlag) -> Self {
        let mut s = self.clone();
        s.flag = flag;
        s
    }
}
