//! DORIS ground station dataset matcher
use crate::prelude::DOMES;

#[cfg(doc)]
use crate::prelude::{GroundStation, Record, DORIS};

/// [Matcher] is used to easily identify [GroundStation]s from a [DORIS] [Record].
#[derive(Debug, Clone, PartialEq)]
pub enum Matcher<'a> {
    /// Search by station ID#
    /// ```
    /// use std::str::FromStr;
    /// use doris_rs::prelude::*;
    ///
    /// let doris = DORIS::from_gzip_file("data/DOR/V3/cs2rx18164.gz")
    ///     .unwrap();
    ///
    /// // Because Station IDs are arbitrary and file dependent,
    /// // it should be very rare to use this search method.
    /// let station_13 = Matcher::ID(13);
    ///
    /// let toulouse = GroundStation::default()
    ///     .with_domes(DOMES::from_str("10003S005").unwrap())
    ///     .with_site_name("TOULOUSE") // site name
    ///     .with_site_label("TLSB")    // site label/mnemonic
    ///     .with_unique_id(13)         // file dependent
    ///     .with_frequency_shift(0)    // f1/f2 site shift for this day
    ///     .with_beacon_revision(3);   // DORIS 3rd generation
    ///
    /// // Stations helper (example)
    /// assert_eq!(doris.ground_station(station_13), Some(toulouse));
    /// ```
    ID(u16),

    /// Search by site name
    /// ```
    /// use std::str::FromStr;
    /// use doris_rs::prelude::*;
    ///
    /// let doris = DORIS::from_gzip_file("data/DOR/V3/cs2rx18164.gz")
    ///     .unwrap();
    ///
    /// // Match by site name (full name), case insensitive.
    /// let to_match = Matcher::Site("toulouse");
    ///
    /// let toulouse = GroundStation::default()
    ///     .with_domes(DOMES::from_str("10003S005").unwrap())
    ///     .with_site_name("TOULOUSE") // site name
    ///     .with_site_label("TLSB")    // site label/mnemonic
    ///     .with_unique_id(13)         // file dependent
    ///     .with_frequency_shift(0)    // f1/f2 site shift for this day
    ///     .with_beacon_revision(3);   // DORIS 3rd generation
    ///
    /// // Stations helper (example)
    /// assert_eq!(doris.ground_station(to_match), Some(toulouse));
    /// ```
    Site(&'a str),

    /// Search by station (mnemonic) label
    /// ```
    /// use std::str::FromStr;
    /// use doris_rs::prelude::*;
    ///
    /// let doris = DORIS::from_gzip_file("data/DOR/V3/cs2rx18164.gz")
    ///     .unwrap();
    ///
    /// // Match by site name (full name), case insensitive.
    /// let to_match = Matcher::Label("tlsb");
    ///
    /// let toulouse = GroundStation::default()
    ///     .with_domes(DOMES::from_str("10003S005").unwrap())
    ///     .with_site_name("TOULOUSE") // site name
    ///     .with_site_label("TLSB")    // site label/mnemonic
    ///     .with_unique_id(13)         // file dependent
    ///     .with_frequency_shift(0)    // f1/f2 site shift for this day
    ///     .with_beacon_revision(3);   // DORIS 3rd generation
    ///
    /// // Stations helper (example)
    /// assert_eq!(doris.ground_station(to_match), Some(toulouse));
    /// ```
    Label(&'a str),

    /// Search by DOMES code
    /// ```
    /// use std::str::FromStr;
    /// use doris_rs::prelude::*;
    ///
    /// let doris = DORIS::from_gzip_file("data/DOR/V3/cs2rx18164.gz")
    ///     .unwrap();
    ///
    /// // Match by site name (full name), case insensitive.
    /// let domes = DOMES::from_str("10003S005")
    ///     .unwrap();
    ///
    /// assert_eq!(domes.area, 100);
    /// assert_eq!(domes.site, 3);
    /// assert_eq!(domes.sequential, 5);
    /// assert_eq!(domes.point, DOMESTrackingPoint::Instrument);
    ///
    /// let to_match = Matcher::DOMES(domes);
    ///
    /// let toulouse = GroundStation::default()
    ///     .with_domes(DOMES::from_str("10003S005").unwrap())
    ///     .with_site_name("TOULOUSE") // site name
    ///     .with_site_label("TLSB")    // site label/mnemonic
    ///     .with_unique_id(13)         // file dependent
    ///     .with_frequency_shift(0)    // f1/f2 site shift for this day
    ///     .with_beacon_revision(3);   // DORIS 3rd generation
    ///
    /// // Stations helper (example)
    /// assert_eq!(doris.ground_station(to_match), Some(toulouse));
    /// ```
    DOMES(DOMES),
}
