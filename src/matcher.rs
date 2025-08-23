//! DORIS ground station dataset matcher
use crate::prelude::DOMES;

#[cfg(doc)]
use crate::prelude::Observation;

/// [Matcher] is used to easily identify [Observation]s of a specific ground station.
#[derive(Debug, Clone, PartialEq)]
pub enum Matcher<'a> {
    /// Search by station ID#
    /// ```
    /// use doris_rs::prelude:*;
    ///
    /// ```
    ID(u16),

    /// Search by site name
    /// ```
    /// use doris_rs::prelude:*;
    ///
    /// ```
    Site(String),

    /// Search by 4 letter station code
    /// ```
    /// use doris_rs::prelude:*;
    ///
    /// ```
    Code(String),

    /// Search by DOMES code
    /// ```
    /// use doris_rs::prelude:*;
    ///
    /// ```
    DOMES(DOMES),
}
