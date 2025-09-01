//! DORIS ground station dataset matcher
use crate::prelude::DOMES;

#[cfg(doc)]
use crate::prelude::{GroundStation, Record, DORIS};

/// [Matcher] is used to easily identify [GroundStation]s from a [DORIS] [Record].
#[derive(Debug, Clone, PartialEq)]
pub enum Matcher<'a> {
    /// Search by station ID#
    /// ```
    /// use doris_rs::prelude::*;
    ///
    /// ```
    ID(u16),

    /// Search by site name
    /// ```
    /// use doris_rs::prelude::*;
    ///
    /// ```
    Site(&'a str),

    /// Scarch by 4 letter station label
    /// ```
    /// use doris_rs::prelude::*;
    ///
    /// ```
    Label(&'a str),

    /// Search by DOMES code
    /// ```
    /// use doris_rs::prelude::*;
    ///
    /// ```
    DOMES(DOMES),
}
