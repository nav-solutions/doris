mod formatting;
mod key;
mod measurement;
mod parsing;

use std::collections::BTreeMap;

#[cfg(doc)]
use crate::prelude::GroundStation;

use crate::prelude::{Comments, Observable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use key::Key;
pub use measurement::{ClockOffset, Measurements};

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
}
