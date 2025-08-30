#[cfg(doc)]
use crate::prelude::{GroundStation, TimeScale, DORIS};

use crate::prelude::{Duration, Observable, Observation};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
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

/// [DORIS] Measurements (also referred to as "Observations") of a [GroundStation]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Measurements {
    /// Satellite (=measurement system) [ClockOffset].
    pub satellite_clock_offset: ClockOffset,

    /// Observations indexed [Observable]s, measurement unit varies.
    pub observations: HashMap<Observable, Observation>,
}

impl Measurements {
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
}
