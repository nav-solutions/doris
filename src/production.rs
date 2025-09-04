use crate::error::ParsingError;

/// This structure is attached to DORIS file that were named
/// according to the standard convention.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ProductionAttributes {
    /// 3 letter satellite name
    pub satellite: String,

    /// Year of production
    pub year: u32,

    /// Production Day of Year (DOY), assumed past J2000.
    pub doy: u32,

    /// True if this file was gzip compressed
    #[cfg(feature = "flate2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "flate2")))]
    pub gzip_compressed: bool,
}

impl std::fmt::Display for ProductionAttributes {
    #[cfg(not(feature = "flate2"))]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let sat_len = self.satellite.len();
        let mut sat_name = self.satellite[..std::cmp::min(sat_len, 5)].to_string();

        for i in sat_len..5 {
            sat_name.push('X');
        }

        write!(f, "{}{:02}{:03}", sat_name, self.year - 2000, self.doy)
    }

    #[cfg(feature = "flate2")]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let sat_len = self.satellite.len();
        let mut sat_name = self.satellite[..std::cmp::min(sat_len, 5)].to_string();

        for i in sat_len..5 {
            sat_name.push('X');
        }

        let mut extension = "".to_string();

        if self.gzip_compressed {
            extension.push_str(".gz");
        }

        write!(
            f,
            "{}{:02}{:03}{}",
            sat_name,
            self.year - 2000,
            self.doy,
            extension
        )
    }
}

impl std::str::FromStr for ProductionAttributes {
    type Err = ParsingError;

    fn from_str(filename: &str) -> Result<Self, Self::Err> {
        let filename = filename.to_uppercase();

        let name_len = filename.len();

        if name_len != 10 && name_len != 13 {
            return Err(ParsingError::NonStandardFileName);
        }

        let mut doy = 0;
        let mut year = 2000;

        let satellite = filename[..5].to_string();

        if let Ok(y) = filename[5..7].parse::<u32>() {
            year += y;
        }

        if let Ok(day) = filename[7..10].parse::<u32>() {
            doy = day;
        }

        Ok(Self {
            satellite,
            year,
            doy,
            #[cfg(feature = "flate2")]
            gzip_compressed: filename.ends_with(".GZ"),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    #[cfg(feature = "flate2")]
    fn test_prod_attributes() {
        for (filename, sat_name, year, doy, gzip_compressed) in [
            ("cs2rx18164", "CS2RX", 2018, 164, false),
            ("cs2rx18164.gz", "CS2RX", 2018, 164, true),
        ] {
            let prod = ProductionAttributes::from_str(filename).unwrap_or_else(|e| {
                panic!("Failed to \"{}\": {}", filename, e);
            });

            assert_eq!(prod.satellite, sat_name);
            assert_eq!(prod.year, year);
            assert_eq!(prod.doy, doy);
            assert_eq!(prod.gzip_compressed, gzip_compressed);
        }
    }
}
