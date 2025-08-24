mod formatting;
pub mod key;
mod parsing;

use std::collections::BTreeMap;

use crate::prelude::Comments;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use key::Key;

/// [Record] contains all [DORIS] data.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Record {
    /// Comments found "as is" during record parsing
    pub comments: Comments,

    /// Measurements stored by [Key], unit is dependent on [Observable] (physics)
    pub observations: BTreeMap<Key, f64>,
}

impl Record {}
