use thiserror::Error;

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
    #[cfg(feature = "flate2")]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }

    #[cfg(not(feature = "flate2"))]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}

impl std::str::FromStr for ProductionAttributes {
    type Err = ParsingError;

    fn from_str(filename: &str) -> Result<Self, Self::Err> {
        let filename = filename.to_uppercase();

        let name_len = filename.len();

        if name_len != 12 && name_len != 15 {
            return Err(ParsingError::NonStandardFileName);
        }

        let doy = 0;
        let satellite = "Undefined".to_string();

        let offset = filename.find('.').unwrap_or(0);

        let agency = filename[..3].to_string();

        let year = filename[offset + 1..offset + 3]
            .parse::<u32>()
            .map_err(|_| ParsingError::NonStandardFileName)?;

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
}
