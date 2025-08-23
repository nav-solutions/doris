use crate::{
    antex::Record as AntexRecord, clock::Record as ClockRecord, doris::Record as DorisRecord,
    meteo::Record as MeteoRecord, navigation::Record as NavRecord,
    observation::Record as ObservationRecord, prelude::Epoch,
};

use std::collections::BTreeMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod formatting;
mod parsing;

pub mod key;

/// [Record] contains all [DORIS] data.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Record {
    /// Comments found "as is" during record parsing
    pub comments: Comments,

    /// [Observation]s stored indexed by [Key]
    pub observations: BTreeMap<Key, Observation>,
}

impl Record {}
