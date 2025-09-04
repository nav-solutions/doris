#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(doc)]
use crate::prelude::Measurements;

use crate::prelude::{Epoch, EpochFlag};

/// [Key] is used to store [GroundStation]s [Measurements] uniquely.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Key {
    /// [EpochFlag] describing attached data and sampling conditions
    pub flag: EpochFlag,

    /// [Epoch] of measurement
    pub epoch: Epoch,
}
