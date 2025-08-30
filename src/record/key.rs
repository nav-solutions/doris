#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(doc)]
use crate::prelude::Measurements;

use crate::prelude::{Epoch, GroundStation};

/// [Key] is used to store [GroundStation]s [Measurements] uniquely.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Key {
    /// [Epoch] of measurement
    pub epoch: Epoch,

    /// [GroundStation] being measured
    pub station: GroundStation,
}
