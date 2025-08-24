#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(doc)]
use crate::prelude::{Header, Observable};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Key {
    /// ID# of this station. You will have to refer to the [Header] section
    /// to truly identify the ground station.
    pub station_id: u16,

    /// [Observable] describes both physics (measurement unit) and frequency (signal interpretation).
    pub observable: Observable,
}
