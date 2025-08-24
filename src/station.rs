#[cfg(doc)]
use crate::prelude::DORIS;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::prelude::{Matcher, ParsingError, DOMES};

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
    pub(crate) key: u16,
}

impl GroundStation {
    /// Returns true if this [GroundStation] is matched by given [Matcher] specs
    pub fn matches<'a>(&self, matcher: &'a Matcher) -> bool {
        match matcher {
            Matcher::ID(code) => self.key == code,
            Matcher::Site(site) => self.site == site,
            Matcher::DOMES(domes) => self.domes == domes,
            Matcher::Code(label) => self.label == label,
        }
    }

    /// Returns S1 frequency shift for this [GroundStation] in Hertz
    pub fn s1_frequency_shift(&self) -> f64 {
        543.0 * Self::USO_FREQ * (3.0 / 4.0 + 87.0 * self.k_factor as f64 / 5.0 * 2.0_f64.powi(26))
    }

    /// Returns U2 frequency shift for this [GroundStation] in Hertz
    pub fn u2_frequency_shift(&self) -> f64 {
        107.0 * Self::USO_FREQ * (3.0 / 4.0 + 87.0 * self.k_factor as f64 / 5.0 * 2.0_f64.powi(26))
    }
}

impl std::str::FromStr for GroundStation {
    type Err = ParsingError;
    fn from_str(content: &str) -> Result<Self, Self::Err> {
        if content.len() < 40 {
            return Err(ParsingError::DorisGroundStationFormat);
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
            gen: gen
                .trim()
                .parse::<u8>()
                .map_err(|_| ParsingError::DorisGroundStation)?,
            k_factor: k_factor
                .trim()
                .parse::<i8>()
                .map_err(|_| ParsingError::DorisGroundStation)?,
            key: key
                .trim()
                .parse::<u16>()
                .map_err(|_| ParsingError::DorisGroundStation)?,
        })
    }
}

impl std::fmt::Display for GroundStation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "D{:02}  {} {:<29} {}  {}   {}",
            self.key, self.label, self.site, self.domes, self.gen, self.k_factor
        )
    }
}

#[cfg(test)]
mod test {
    use super::GroundStation;
    use crate::prelude::{DOMESTrackingPoint, DOMES};
    use std::str::FromStr;
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
                    gen: 3,
                    k_factor: 0,
                    key: 1,
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
                    gen: 3,
                    k_factor: 0,
                    key: 17,
                },
            ),
        ] {
            let station = GroundStation::from_str(desc).unwrap();
            assert_eq!(station, expected, "station parsing error");
            assert_eq!(station.to_string(), desc, "station reciprocal error");
        }
    }
}
