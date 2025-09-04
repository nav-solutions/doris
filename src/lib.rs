#![doc(
    html_logo_url = "https://raw.githubusercontent.com/nav-solutions/.github/master/logos/logo2.jpg"
)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::type_complexity)]

/*
 * DORIS is part of the nav-solutions framework.
 * Authors: Guillaume W. Bres <guillaume.bressaix@gmail.com> et al.
 * (cf. https://github.com/nav-solutions/doris/graphs/contributors)
 * This framework is licensed under Mozilla Public V2 license.
 *
 * Documentation: https://github.com/nav-solutions/doris
 */

extern crate num_derive;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

extern crate gnss_rs as gnss;
extern crate num;

pub mod constants;
pub mod error;
pub mod frequency;
pub mod header;
pub mod matcher;
pub mod observable;
pub mod production;
pub mod record;
pub mod station;

mod epoch;

#[cfg(test)]
mod tests;

use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
    str::FromStr,
};

use itertools::Itertools;

#[cfg(feature = "flate2")]
use flate2::{read::GzDecoder, write::GzEncoder, Compression as GzCompression};

use hifitime::prelude::{Duration, Epoch};

use crate::{
    error::{Error, FormattingError, ParsingError},
    header::Header,
    matcher::Matcher,
    observable::Observable,
    production::ProductionAttributes,
    record::{ClockOffset, Record},
    station::GroundStation,
};

/// [Comments] found in [DORIS] files
pub type Comments = Vec<String>;

pub mod prelude {
    // export
    pub use crate::{
        error::{FormattingError, ParsingError},
        frequency::Frequency,
        header::{Antenna, Header, Receiver, Version},
        matcher::Matcher,
        observable::Observable,
        production::ProductionAttributes,
        record::{
            ClockOffset, EpochFlag, Key, Measurements, Observation, ObservationKey, Record, SNR,
        },
        station::GroundStation,
        Comments, DORIS,
    };

    pub use gnss::prelude::{Constellation, DOMESTrackingPoint, COSPAR, DOMES, SV};

    pub use hifitime::{Duration, Epoch, Polynomial, TimeScale, TimeSeries};
}

pub(crate) fn fmt_doris(content: &str, marker: &str) -> String {
    if content.len() < 60 {
        format!("{:<padding$}{}", content, marker, padding = 60)
    } else {
        let mut string = String::new();
        let nb_lines = num_integer::div_ceil(content.len(), 60);
        for i in 0..nb_lines {
            let start_off = i * 60;
            let end_off = std::cmp::min(start_off + 60, content.len());
            let chunk = &content[start_off..end_off];
            string.push_str(&format!("{:<padding$}{}", chunk, marker, padding = 60));
            if i < nb_lines - 1 {
                string.push('\n');
            }
        }
        string
    }
}

pub(crate) fn fmt_comment(content: &str) -> String {
    fmt_doris(content, "COMMENT")
}

#[derive(Clone, Default, Debug, PartialEq)]
/// [DORIS] is composed of a [Header] and a [Record] section.
/// ```
/// ```
pub struct DORIS {
    /// [Header] gives general information
    pub header: Header,

    /// [Record] gives the actual file content
    pub record: Record,

    /// [ProductionAttributes] is attached to files that were
    /// named according to the standard conventions.
    pub production: Option<ProductionAttributes>,
}

impl DORIS {
    /// Builds a new [DORIS] struct from given [Header] and [Record] sections.
    pub fn new(header: Header, record: Record) -> DORIS {
        DORIS {
            header,
            record,
            production: Default::default(),
        }
    }

    /// Copy and return this [DORIS] with updated [Header].
    pub fn with_header(&self, header: Header) -> Self {
        Self {
            header,
            record: self.record.clone(),
            production: Default::default(),
        }
    }

    /// Replace [Header] with mutable access.
    pub fn replace_header(&mut self, header: Header) {
        self.header = header.clone();
    }

    /// Copies and returns a [DORIS] with updated [Record]
    pub fn with_record(&self, record: Record) -> Self {
        DORIS {
            record,
            header: self.header.clone(),
            production: self.production.clone(),
        }
    }

    /// Replace [Record] with mutable access.
    pub fn replace_record(&mut self, record: Record) {
        self.record = record.clone();
    }

    /// Parse [DORIS] content by consuming [BufReader] (efficient buffered reader).
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParsingError> {
        // Parses Header section (=consumes header until this point)
        let mut header = Header::parse(reader)?;

        // Parse record (=consumes rest of this resource)
        // Comments are preserved and store "as is"
        let record = Record::parse(&mut header, reader)?;

        Ok(Self {
            header,
            record,
            production: Default::default(),
        })
    }

    /// Format [DORIS] into writable I/O using efficient buffered writer
    /// and following standard specifications. This is the mirror operation of [Self::parse].
    pub fn format<W: Write>(&self, writer: &mut BufWriter<W>) -> Result<(), FormattingError> {
        self.header.format(writer)?;
        self.record.format(writer, &self.header)?;
        writer.flush()?;
        Ok(())
    }

    /// Parses [DORIS] from local readable file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<DORIS, ParsingError> {
        let path = path.as_ref();

        // deduce all we can from file name
        let file_attributes = match path.file_name() {
            Some(filename) => {
                let filename = filename.to_string_lossy().to_string();
                if let Ok(prod) = ProductionAttributes::from_str(&filename) {
                    Some(prod)
                } else {
                    None
                }
            },
            _ => None,
        };

        let fd = File::open(path)?;

        let mut reader = BufReader::new(fd);
        let mut doris = Self::parse(&mut reader)?;

        doris.production = file_attributes;

        Ok(doris)
    }

    /// Dumps [DORIS] into writable local file (as readable ASCII UTF-8)
    /// using efficient buffered formatting.
    /// This is the mirror operation of [Self::from_file].
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), FormattingError> {
        let fd = File::create(path)?;
        let mut writer = BufWriter::new(fd);
        self.format(&mut writer)?;
        Ok(())
    }

    /// Parses [DORIS] from local gzip compressed file.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use doris_rs::prelude::*;
    ///
    /// let doris = DORIS::from_gzip_file("data/DOR/V3/cs2rx18164.gz")
    ///     .unwrap();
    ///
    /// assert_eq!(doris.header.satellite, "CRYOSAT-2");
    ///
    /// let agency = "CNES".to_string(); // Agency / producer
    /// let program = "Expert".to_string(); // Software name
    /// let run_by = "CNES".to_string(); // Operator
    /// let date = "20180614 090016 UTC".to_string(); // Date of production
    /// let observer = "SPA_BN1_4.7P1".to_string(); // Operator
    ///
    /// assert_eq!(doris.header.program, Some(program));
    /// assert_eq!(doris.header.run_by, Some(run_by));
    /// assert_eq!(doris.header.date, Some(date)); // currently not interpreted
    /// assert_eq!(doris.header.observer, Some(observer));
    /// assert_eq!(doris.header.agency, Some(agency));
    ///
    /// assert!(doris.header.doi.is_none());
    /// assert!(doris.header.license.is_none());
    ///
    /// let observables = vec![
    ///    Observable::UnambiguousPhaseRange(Frequency::DORIS1), // phase, in meters of prop.
    ///    Observable::UnambiguousPhaseRange(Frequency::DORIS2),
    ///    Observable::PseudoRange(Frequency::DORIS1), // decoded pseudo range
    ///    Observable::PseudoRange(Frequency::DORIS2),
    ///    Observable::Power(Frequency::DORIS1), // received power
    ///    Observable::Power(Frequency::DORIS2), // received power
    ///    Observable::FrequencyRatio,           // f1/f2 ratio (=drift image)
    ///    Observable::Pressure,                 // pressure, at ground station level (hPa)
    ///    Observable::Temperature,              // temperature, at ground station level (°C)
    ///    Observable::HumidityRate,             // saturation rate, at ground station level (%)
    /// ];
    ///
    /// assert_eq!(doris.header.observables, observables);
    ///
    /// assert_eq!(doris.header.ground_stations.len(), 53); // network
    ///
    /// // Stations helper
    /// let site_matcher = Matcher::Site("TOULOUSE");
    ///
    /// let toulouse = GroundStation::default()
    ///     .with_domes(DOMES::from_str("10003S005").unwrap())
    ///     .with_site_name("TOULOUSE") // site name
    ///     .with_site_label("TLSB")    // site label/mnemonic
    ///     .with_unique_id(13)         // file dependent
    ///     .with_frequency_shift(0)    // f1/f2 site shift for this day
    ///     .with_beacon_revision(3);   // DORIS 3rd generation
    ///
    /// // helper
    /// assert_eq!(doris.ground_station(site_matcher), Some(toulouse));
    /// ```
    #[cfg(feature = "flate2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "flate2")))]
    pub fn from_gzip_file<P: AsRef<Path>>(path: P) -> Result<DORIS, ParsingError> {
        let path = path.as_ref();

        // deduce all we can from file name
        let file_attributes = match path.file_name() {
            Some(filename) => {
                let filename = filename.to_string_lossy().to_string();
                if let Ok(prod) = ProductionAttributes::from_str(&filename) {
                    Some(prod)
                } else {
                    None
                }
            },
            _ => None,
        };

        let fd = File::open(path)?;

        let reader = GzDecoder::new(fd);
        let mut reader = BufReader::new(reader);
        let mut doris = Self::parse(&mut reader)?;

        doris.production = file_attributes;

        Ok(doris)
    }

    /// Dumps and gzip encodes [DORIS] into writable local file,
    /// using efficient buffered formatting.
    /// This is the mirror operation of [Self::from_gzip_file].
    #[cfg(feature = "flate2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "flate2")))]
    pub fn to_gzip_file<P: AsRef<Path>>(&self, path: P) -> Result<(), FormattingError> {
        let fd = File::create(path)?;
        let compression = GzCompression::new(5);
        let mut writer = BufWriter::new(GzEncoder::new(fd, compression));
        self.format(&mut writer)?;
        Ok(())
    }

    /// Determines whether this structure results of combining several structures
    /// into a single one. This is determined by the presence of a custom yet somewhat standardized Header comment.
    pub fn is_merged(&self) -> bool {
        let special_comment = String::from("FILE MERGE");

        for comment in self.header.comments.iter() {
            if comment.eq("FILE MERGE") {
                return true;
            }
        }

        false
    }

    /// Returns [GroundStation] information for matching site
    pub fn ground_station<'a>(&self, matcher: Matcher<'a>) -> Option<GroundStation> {
        self.header
            .ground_stations
            .iter()
            .filter(|station| station.matches(&matcher))
            .reduce(|k, _| k)
            .cloned()
    }

    /// Returns measurement satellite [ClockOffset] [Iterator] for all Epochs, in chronological order
    ///
    /// ```
    /// use doris_rs::prelude::*;
    ///
    /// let doris = DORIS::from_gzip_file("data/DOR/V3/cs2rx18164.gz")
    ///     .unwrap();
    ///
    /// assert_eq!(doris.header.satellite, "CRYOSAT-2");
    ///
    /// for (i, (epoch, clock_offset)) in doris.satellite_clock_offset_iter().enumerate() {
    ///
    ///     assert_eq!(clock_offset.extrapolated, false); // actual measurement
    ///
    ///     if i == 0 {
    ///         assert_eq!(clock_offset.offset.to_seconds(), -4.326631626);
    ///     } else if i == 10 {
    ///         assert_eq!(clock_offset.offset.to_seconds(), -4.326631711);
    ///     }
    /// }
    /// ```
    pub fn satellite_clock_offset_iter(
        &self,
    ) -> Box<dyn Iterator<Item = (Epoch, ClockOffset)> + '_> {
        Box::new(
            self.record
                .measurements
                .iter()
                .filter_map(|(k, v)| {
                    if let Some(clock_offset) = v.satellite_clock_offset {
                        Some((k.epoch, clock_offset))
                    } else {
                        None
                    }
                })
                .unique(),
        )
    }

    /// Returns histogram analysis of the sampling period, as ([Duration], population [usize]) tuple.
    /// ```
    /// use doris_rs::prelude::*;
    /// use itertools::Itertools;
    ///
    /// let doris = DORIS::from_gzip_file("data/DOR/V3/cs2rx18164.gz")
    ///     .unwrap();
    ///
    /// // requires more than 2 measurements
    /// let (sampling_period, population) = doris.sampling_histogram()
    ///     .sorted()
    ///     .nth(0) // dominant
    ///     .unwrap();
    ///
    /// assert_eq!(sampling_period, Duration::from_seconds(3.0));
    /// ```
    pub fn sampling_histogram(&self) -> Box<dyn Iterator<Item = (Duration, usize)> + '_> {
        Box::new(
            self.record
                .epochs_iter()
                .zip(self.record.epochs_iter().skip(1))
                .map(|((ek_1, _), (ek_2, _))| ek_2 - ek_1)
                .fold(vec![], |mut list, dt| {
                    let mut found = false;

                    for (delta, pop) in list.iter_mut() {
                        if *delta == dt {
                            *pop += 1;
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        list.push((dt, 1));
                    }

                    list
                })
                .into_iter(),
        )
    }

    /// Studies actual measurement rate and returns the highest
    /// value in the histogram as the dominant sampling rate
    ///
    /// ```
    /// use doris_rs::prelude::*;
    /// use itertools::Itertools;
    ///
    /// let doris = DORIS::from_gzip_file("data/DOR/V3/cs2rx18164.gz")
    ///     .unwrap();
    ///
    /// // requires more than 2 measurements
    /// let sampling_period = doris.dominant_sampling_period()
    ///     .unwrap();
    ///
    /// assert_eq!(sampling_period, Duration::from_seconds(3.0));
    /// ```
    pub fn dominant_sampling_period(&self) -> Option<Duration> {
        self.sampling_histogram()
            .sorted()
            .map(|(dt, _)| dt)
            .reduce(|k, _| k)
    }

    /// Generates (guesses) a standardized (uppercase) filename from this actual [DORIS] data set.
    /// This is particularly useful when initiated from a file that did not follow
    /// standard naming conventions.
    ///
    /// ```
    /// use doris_rs::prelude::*;
    ///
    /// // parse standard file
    /// let doris = DORIS::from_gzip_file("data/DOR/V3/cs2rx18164.gz")
    ///     .unwrap();
    ///
    /// assert_eq!(doris.standard_filename(), "CS2RX18164.gz");
    ///
    /// // Dump using random name
    /// doris.to_file("example.txt")
    ///     .unwrap();
    ///
    /// // parse back & use
    /// let parsed = DORIS::from_file("example.txt")
    ///     .unwrap();
    ///
    /// assert_eq!(parsed.header.satellite, "CRYOSAT-2");
    ///
    /// // when coming from non standard names,
    /// // all fields are deduced from actual content.
    /// assert_eq!(parsed.standard_filename(), "CRYOS18164");
    /// ```
    pub fn standard_filename(&self) -> String {
        let mut doy = 0;
        let mut year = 0i32;
        let mut extension = "".to_string();

        let sat_len = self.header.satellite.len();
        let mut sat_name = self.header.satellite[..std::cmp::min(sat_len, 5)].to_string();

        if let Some(epoch) = self.header.time_of_first_observation {
            year = epoch.year() - 2000;
            doy = epoch.day_of_year().round() as u32;
        }

        if let Some(attributes) = &self.production {
            doy = attributes.doy;
            year = attributes.year as i32 - 2000;

            let sat_len = attributes.satellite.len();
            sat_name = String::from(&attributes.satellite[..std::cmp::min(sat_len, 5)]);

            #[cfg(feature = "flate2")]
            if attributes.gzip_compressed {
                extension.push_str(".gz");
            }
        }

        for i in sat_len..5 {
            sat_name.push('X');
        }

        format!("{}{:02}{:03}{}", sat_name, year, doy, extension)
    }

    /// Copies and returns new [DORIS] that is the result
    /// of ground station observation differentiation.
    /// See [Self::observations_substract_mut] for more information.
    pub fn substract(&self, rhs: &Self) -> Result<Self, Error> {
        let mut s = self.clone();
        s.substract_mut(rhs)?;
        Ok(s)
    }

    /// Substract (in place) this [DORIS] file to another, creating
    /// a "residual" [DORIS] file. All common and synchronous measurements
    /// are substracted to one another, others are discarded and dropped
    /// after this operation.
    pub fn substract_mut(&mut self, rhs: &Self) -> Result<(), Error> {
        let lhs_dt = self
            .dominant_sampling_period()
            .ok_or(Error::UndeterminedSamplingRate)?;

        let half_lhs_dt = lhs_dt / 2.0;

        // if let Some(rhs) = rhs.record.as_obs() {
        //     if let Some(rec) = self.record.as_mut_obs() {
        //         rec.retain(|k, v| {
        //             v.signals.retain_mut(|sig| {
        //                 let mut reference = 0.0;
        //                 let mut min_dt = Duration::MAX;

        //                 // temporal filter
        //                 let filtered_rhs_epochs = rhs.iter().filter(|(rhs, _)| {
        //                     let dt = (rhs.epoch - k.epoch).abs();
        //                     dt <= half_lhs_dt
        //                 });

        //                 for (rhs_epoch, rhs_values) in filtered_rhs_epochs {
        //                     for rhs_sig in rhs_values.signals.iter() {
        //                         if rhs_sig.sv == sig.sv && rhs_sig.observable == sig.observable {
        //                             let dt = (rhs_epoch.epoch - k.epoch).abs();
        //                             if dt <= min_dt {
        //                                 reference = rhs_sig.value;
        //                                 min_dt = dt;
        //                             }
        //                         }
        //                     }
        //                 }

        //                 if min_dt < Duration::MAX {
        //                     sig.value -= reference;
        //                 }

        //                 min_dt < Duration::MAX
        //             });

        //             !v.signals.is_empty()
        //         });
        //     }
        // }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fmt_comment;

    #[test]
    fn fmt_comments_singleline() {
        for desc in [
            "test",
            "just a basic comment",
            "just another lengthy comment blahblabblah",
        ] {
            let comment = fmt_comment(desc);
            assert!(
                comment.len() >= 60,
                "comments should be at least 60 byte long"
            );

            assert_eq!(
                comment.find("COMMENT"),
                Some(60),
                "comment marker should located @ 60"
            );
        }
    }

    #[test]
    fn fmt_wrapped_comments() {
        for desc in ["just trying to form a very lengthy comment that will overflow since it does not fit in a single line",
            "just trying to form a very very lengthy comment that will overflow since it does fit on three very meaningful lines. Imazdmazdpoakzdpoakzpdokpokddddddddddddddddddaaaaaaaaaaaaaaaaaaaaaaa"] {
            let nb_lines = num_integer::div_ceil(desc.len(), 60);
            let comments = fmt_comment(desc);
            assert_eq!(comments.lines().count(), nb_lines);
            for line in comments.lines() {
                assert!(line.len() >= 60, "comment line should be at least 60 byte long");
                assert_eq!(line.find("COMMENT"), Some(60), "comment marker should located @ 60");
            }
        }
    }

    #[test]
    fn fmt_observables_v3() {
        for (desc, expected) in [
("R    9 C1C L1C S1C C2C C2P L2C L2P S2C S2P",
"R    9 C1C L1C S1C C2C C2P L2C L2P S2C S2P                  SYS / # / OBS TYPES"),
("G   18 C1C L1C S1C C2P C2W C2S C2L C2X L2P L2W L2S L2L L2X         S2P S2W S2S S2L S2X",
"G   18 C1C L1C S1C C2P C2W C2S C2L C2X L2P L2W L2S L2L L2X  SYS / # / OBS TYPES
       S2P S2W S2S S2L S2X                                  SYS / # / OBS TYPES"),
        ] {
            assert_eq!(fmt_doris(desc, "SYS / # / OBS TYPES"), expected);
        }
    }
}
