#[cfg(doc)]
use crate::prelude::{GroundStation, TimeScale, DORIS};

use crate::{
    error::ParsingError,
    prelude::{ClockOffset, Duration, GroundStation, Matcher, Observable, Observation},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use itertools::Itertools;
use std::collections::BTreeMap;

/// [ObservationKey] is used to store [GroundStation]s observations uniquely
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ObservationKey {
    /// [GroundStation] being observed
    pub station: GroundStation,

    /// [Observable] determines the physics and measurement unit
    pub observable: Observable,
}

/// [DORIS] Measurements (also referred to as "Observations") of a [GroundStation]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Measurements {
    /// Satellite (=measurement system) [ClockOffset].
    pub satellite_clock_offset: Option<ClockOffset>,

    /// Observations indexed [Observable]s, measurement unit varies.
    pub observations: BTreeMap<ObservationKey, Observation>,
}

impl Measurements {
    /// Add a new observation to this set of [Measurements]  
    pub fn add_observation(
        &mut self,
        station: GroundStation,
        observable: Observable,
        observation: Observation,
    ) {
        self.observations.insert(
            ObservationKey {
                station,
                observable,
            },
            observation,
        );
    }

    /// Updates this set of [Measurements] with a new observation
    pub fn with_observation(
        &self,
        station: GroundStation,
        observable: Observable,
        observation: Observation,
    ) -> Self {
        let mut s = self.clone();
        s.observations.insert(
            ObservationKey {
                station,
                observable,
            },
            observation,
        );
        s
    }

    /// Returns list of [Observable]s measured at this epoch, regardless of the observed site.
    pub fn observables(&self) -> Box<dyn Iterator<Item = Observable> + '_> {
        Box::new(self.observations.keys().map(|k| k.observable).unique())
    }

    /// Returns list of [Observable]s measured for this given site at attached epoch.
    pub fn station_observables<'a>(
        &'a self,
        matcher: &'a Matcher<'a>,
    ) -> Box<dyn Iterator<Item = Observable> + '_> {
        Box::new(
            self.observations
                .keys()
                .filter_map(|k| {
                    if k.station.matches(matcher) {
                        Some(k.observable)
                    } else {
                        None
                    }
                })
                .unique(),
        )
    }

    /// Copies and returns [Measurements] with updated [ClockOffset]
    pub fn with_satellite_clock_offset(&self, clock_offset: ClockOffset) -> Self {
        let mut s = self.clone();
        s.satellite_clock_offset = Some(clock_offset);
        s
    }
}
