mod clock;
mod flag;
mod formatting;
mod key;
mod measurement;
mod observation;
mod parsing;
mod snr;

use itertools::Itertools;
use std::collections::BTreeMap;

#[cfg(doc)]
use crate::prelude::GroundStation;

use crate::prelude::{Comments, Epoch, Matcher, Observable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use clock::ClockOffset;
pub use flag::EpochFlag;
pub use key::Key;
pub use measurement::Measurements;
pub use observation::Observation;
pub use snr::SNR;

/// [Record] contains all [DORIS] data.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Record {
    /// Comments found "as is" during record parsing
    pub comments: Comments,

    /// [GroundStation]s [Measurement]s, in chronolical order.
    /// Observations vary with the satellite orbit course.
    pub measurements: BTreeMap<Key, Measurements>,
}

impl Record {
    /// Obtain a chronological ([Epoch], and [EpochFlag]) [Iterator]
    pub fn epochs_iter(&self) -> Box<dyn Iterator<Item = (Epoch, EpochFlag)> + '_> {
        Box::new(self.measurements.keys().map(|k| (k.epoch, k.flag)).unique())
    }

    /// Returns the list of [Observable]s for given station
    pub fn station_observables_iter<'a>(
        &'a self,
        matcher: &'a Matcher<'a>,
    ) -> Box<dyn Iterator<Item = Observable> + '_> {
        Box::new(
            self.measurements
                .iter()
                .flat_map(move |(k, v)| {
                    v.observations.keys().filter_map(move |observable| {
                        if k.station.matches(&matcher) {
                            Some(*observable)
                        } else {
                            None
                        }
                    })
                })
                .unique(),
        )
    }
}
