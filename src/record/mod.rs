mod clock;
mod flag;
mod formatting;
mod key;
mod measurement;
mod observation;
mod parsing;
mod snr;

use std::collections::BTreeMap;

#[cfg(doc)]
use crate::prelude::GroundStation;

use crate::prelude::{Comments, Epoch, Observable};

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
    /// Returns a unique list of [Observable]s, defining all physics
    /// measured for this set of [GroundStation]s.
    pub fn observables(&self) -> Box<dyn Iterator<Item = Observable> + '_> {
        Box::new([].into_iter())
    }

    /// Obtain a chronological ([Epoch], and [EpochFlag]) [Iterator]
    pub fn epochs_iter(&self) -> Box<dyn Iterator<Item = (Epoch, EpochFlag)> + '_> {
        Box::new([].into_iter())
    }
}
