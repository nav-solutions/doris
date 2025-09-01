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

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
