use std::str::FromStr;

#[cfg(doc)]
use crate::prelude::DORIS;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    constants::USO_FREQ_HZ,
    prelude::{Matcher, ParsingError, DOMES},
};

/// [GroundStation] definition, observed from DORIS satellites.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GroundStation {
    /// 4 letter station mnemonic label (antenna point)
    pub label: String,

    /// Site name
    pub site: String,

    /// [DOMES] site identifier
    pub domes: DOMES,

    /// [DORIS] beacon generation
    pub beacon_revision: u8,

    /// K frequency shift factor
    pub k_frequency_shift: i8,

    /// ID# used in file indexing
    pub(crate) code: u16,
}

impl Default for GroundStation {
    /// Builds a default [GroundStation] which not suitable as is
    /// and must be customized.
    fn default() -> Self {
        Self {
            label: Default::default(),
            site: Default::default(),
            domes: DOMES::from_str("10003S005").unwrap(),
            beacon_revision: 3,
            k_frequency_shift: 0,
            code: 0,
        }
    }
}

impl GroundStation {
    /// Defines a [GroundStation] with desired site name
    pub fn with_site_name(&self, name: &str) -> Self {
        let mut s = self.clone();
        s.site = name.to_string();
        s
    }

    /// Defines a [GroundStation] with desired site label,
    /// which should be a 4 letter description of the site name.
    pub fn with_site_label(&self, label: &str) -> Self {
        let mut s = self.clone();
        s.label = label.to_string();
        s
    }

    /// Defines a [GroundStation] with desired [DOMES] site number
    pub fn with_domes_str(&self, domes: &str) -> Result<Self, ParsingError> {
        let domes = DOMES::from_str(domes)?;
        Ok(self.with_domes(domes))
    }

    /// Defines a [GroundStation] with desired [DOMES] site number
    pub fn with_domes(&self, domes: DOMES) -> Self {
        let mut s = self.clone();
        s.domes = domes;
        s
    }

    /// Returns [GroundStation] with updated DORIS beacon revision
    pub fn with_beacon_revision(&self, revision: u8) -> Self {
        let mut s = self.clone();
        s.beacon_revision = revision;
        s
    }

    /// Defines a [GroundStation] with updated f1/f2 frequency shift
    pub fn with_frequency_shift(&self, shift: i8) -> Self {
        let mut s = self.clone();
        s.k_frequency_shift = shift;
        s
    }

    /// Defines a [GroundStation] with desired station ID#
    /// which is used to define this site uniquely in a DORIS file.
    pub fn with_unique_id(&self, code: u16) -> Self {
        let mut s = self.clone();
        s.code = code;
        s
    }

    /// Returns true if this [GroundStation] is matched by given [Matcher] specs
    pub fn matches<'a>(&self, matcher: &'a Matcher) -> bool {
        match matcher {
            Matcher::ID(code) => self.code == *code,
            Matcher::Site(site) => self.site == *site,
            Matcher::DOMES(domes) => self.domes == *domes,
            Matcher::Label(label) => self.label == *label,
        }
    }

    /// Returns S1 frequency shift for this [GroundStation] in Hertz
    pub fn s1_frequency_shift(&self) -> f64 {
        543.0
            * USO_FREQ_HZ
            * (3.0 / 4.0 + 87.0 * self.k_frequency_shift as f64 / 5.0 * 2.0_f64.powi(26))
    }

    /// Returns U2 frequency shift for this [GroundStation] in Hertz
    pub fn u2_frequency_shift(&self) -> f64 {
        107.0
            * USO_FREQ_HZ
            * (3.0 / 4.0 + 87.0 * self.k_frequency_shift as f64 / 5.0 * 2.0_f64.powi(26))
    }
}

impl std::str::FromStr for GroundStation {
    type Err = ParsingError;
    fn from_str(content: &str) -> Result<Self, Self::Err> {
        if content.len() < 40 {
            return Err(ParsingError::GroundStation);
        }

        let content = content.split_at(1).1;
        let (key, rem) = content.split_at(4);
        let (label, rem) = rem.split_at(5);
        let (name, rem) = rem.split_at(30);
        let (domes, rem) = rem.split_at(10);
        let (gen, rem) = rem.split_at(3);
        let (k_factor, _) = rem.split_at(3);

        Ok(GroundStation {
            site: name.trim().to_string(),
            label: label.trim().to_string(),
            domes: DOMES::from_str(domes.trim())?,
            beacon_revision: gen
                .trim()
                .parse::<u8>()
                .map_err(|_| ParsingError::GroundStation)?,
            k_frequency_shift: k_factor
                .trim()
                .parse::<i8>()
                .map_err(|_| ParsingError::GroundStation)?,
            code: key
                .trim()
                .parse::<u16>()
                .map_err(|_| ParsingError::GroundStation)?,
        })
    }
}

impl std::fmt::Display for GroundStation {
    /// Formats [GroundStation] verbosely
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Station {} ({}/{}) (rev={}) (freq={})",
            self.label, self.site, self.domes, self.beacon_revision, self.k_frequency_shift
        )
    }
}

impl std::fmt::LowerHex for GroundStation {
    /// Formats [GroundStation] according to DORIS standards
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "D{:02}  {} {:<29} {}  {} {:3}",
            self.code,
            self.label,
            self.site,
            self.domes,
            self.beacon_revision,
            self.k_frequency_shift
        )
    }
}

#[cfg(test)]
mod test {
    use super::GroundStation;
    use crate::prelude::{DOMESTrackingPoint, DOMES};
    use std::str::FromStr;

    #[test]
    fn default_station() {
        let _ = GroundStation::default();
    }

    #[test]
    fn station_parsing() {
        for (desc, expected) in [
            (
                "D01  OWFC OWENGA                        50253S002  3   0",
                GroundStation {
                    label: "OWFC".to_string(),
                    site: "OWENGA".to_string(),
                    domes: DOMES {
                        area: 502,
                        site: 53,
                        sequential: 2,
                        point: DOMESTrackingPoint::Instrument,
                    },
                    beacon_revision: 3,
                    k_frequency_shift: 0,
                    code: 1,
                },
            ),
            (
                "D17  GRFB GREENBELT                     40451S178  3   0",
                GroundStation {
                    label: "GRFB".to_string(),
                    site: "GREENBELT".to_string(),
                    domes: DOMES {
                        area: 404,
                        site: 51,
                        sequential: 178,
                        point: DOMESTrackingPoint::Instrument,
                    },
                    beacon_revision: 3,
                    k_frequency_shift: 0,
                    code: 17,
                },
            ),
            (
                "D12  GR4B GRASSE                        10002S019  3 -15",
                GroundStation {
                    label: "GR4B".to_string(),
                    site: "GRASSE".to_string(),
                    domes: DOMES {
                        area: 100,
                        site: 02,
                        sequential: 19,
                        point: DOMESTrackingPoint::Instrument,
                    },
                    beacon_revision: 3,
                    k_frequency_shift: -15,
                    code: 12,
                },
            ),
        ] {
            let station = GroundStation::from_str(desc).unwrap();
            assert_eq!(station, expected, "station parsing error");

            let formatted = format!("{:x}", station);
            assert_eq!(formatted, desc, "station reciprocal error");
        }
    }
}
