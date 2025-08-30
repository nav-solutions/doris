mod formatting;
mod parsing;

mod antenna;
mod receiver;
mod version;

use itertools::Itertools;
use std::collections::HashMap;

use crate::{
    prelude::{Duration, Epoch, GroundStation, Observable, COSPAR, DOMES},
    Comments,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use antenna::Antenna;
pub use receiver::Receiver;
pub use version::Version;

/// DORIS [Header]
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Header {
    /// [Version] describes the file revision
    pub version: Version,

    /// Comments found in the [Header] section
    pub comments: Comments,

    /// Name of the DORIS satellite
    pub satellite: String,

    /// Software program name.
    pub program: Option<String>,

    /// Software operator.
    pub run_by: Option<String>,

    /// Date of publication.
    pub date: Option<String>,

    /// Observer name
    pub observer: Option<String>,

    /// Production agency
    pub agency: Option<String>,

    /// Possible COSPAR number (launch information)
    pub cospar: Option<COSPAR>,

    /// Possible information about [Receiver] hardware
    pub receiver: Option<Receiver>,

    /// Possible information about receiver [Antenna]
    pub antenna: Option<Antenna>,

    /// Possible file license
    pub license: Option<String>,

    /// Possible Digital Object Identifier (DOI)
    pub doi: Option<String>,

    /// Possible scalings to apply to attached [Observable]s
    pub scaling_factors: HashMap<Observable, f64>,

    /// DORIS L1/L2 date offset
    pub l1_l2_date_offset: Duration,

    /// DORIS [GroundStation]s
    pub ground_stations: Vec<GroundStation>,

    /// Possible indication of the first measurement
    pub time_of_first_observation: Option<Epoch>,

    /// Possible indication of the last measurement
    pub time_of_last_observation: Option<Epoch>,
}

impl Header {
    /// Identify a [GroundStation] from [u16] (unique) identification
    /// code, which is file or network dependent.
    pub fn ground_station(&self, station_code: u16) -> Option<GroundStation> {
        self.ground_stations
            .iter()
            .filter(|station| station.code == station_code)
            .reduce(|k, _| k)
            .cloned()
    }

    /// Formats the package version (possibly shortenned, in case of lengthy release)
    /// to fit within a formatted COMMENT
    pub(crate) fn format_pkg_version(version: &str) -> String {
        version
            .split('.')
            .enumerate()
            .filter_map(|(nth, v)| {
                if nth < 2 {
                    Some(v.to_string())
                } else if nth == 2 {
                    Some(
                        v.split('-')
                            .filter_map(|v| {
                                if v == "rc" {
                                    Some("rc".to_string())
                                } else {
                                    let mut s = String::new();
                                    s.push_str(&v[0..1]);
                                    Some(s)
                                }
                            })
                            .join(""),
                    )
                } else {
                    None
                }
            })
            .join(".")
    }

    /// Generates the special "FILE MERGE" comment
    pub(crate) fn merge_comment(pkg_version: &str, timestamp: Epoch) -> String {
        let formatted_version = Self::format_pkg_version(pkg_version);

        let (y, m, d, hh, mm, ss, _) = timestamp.to_gregorian_utc();

        format!(
            "doris-rs v{} {:>width$}          {}{:02}{:02} {:02}{:02}{:02} {:x}",
            formatted_version,
            "FILE MERGE",
            y,
            m,
            d,
            hh,
            mm,
            ss,
            timestamp.time_scale,
            width = 19 - formatted_version.len(),
        )
    }

    /// Copies and returns [Header] with specific RINEX [Version]
    pub fn with_version(&self, version: Version) -> Self {
        let mut s = self.clone();
        s.version = version;
        s
    }

    /// Copies and returns [Header] with "Run By" field
    pub fn with_run_by(&self, run_by: &str) -> Self {
        let mut s = self.clone();
        s.run_by = Some(run_by.to_string());
        s
    }

    /// Copies and returns new [Header] with specific [Receiver]
    pub fn with_receiver(&self, receiver: Receiver) -> Self {
        let mut s = self.clone();
        s.receiver = Some(receiver);
        s
    }

    /// Adds one comment to mutable [Self]
    pub fn push_comment(&mut self, comment: &str) {
        self.comments.push(comment.to_string());
    }

    /// Copies and returns [Header] with one new comment.
    pub fn with_comment(&self, comment: &str) -> Self {
        let mut s = self.clone();
        s.comments.push(comment.to_string());
        s
    }
}
