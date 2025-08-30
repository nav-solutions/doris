use crate::{fmt_doris, prelude::FormattingError};

use std::io::{BufWriter, Write};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Antenna description
#[derive(Default, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Antenna {
    /// Name of this [Antenna] model
    pub model: String,

    /// Serial number
    pub serial_number: String,

    /// Approximate coordinates of the anntena base
    pub approx_coordinates: Option<(f64, f64, f64)>,

    /// Optionnal `h` eccentricity (height component),
    /// referenced to base/reference point, in meter
    pub height: Option<f64>,

    /// Optionnal `eastern` eccentricity (eastern component),
    /// referenced to base/reference point, in meter
    pub eastern: Option<f64>,

    /// Optionnal `northern` eccentricity (northern component),
    /// referenced to base/reference point, in meter
    pub northern: Option<f64>,
}

impl Antenna {
    /// Formats [Antenna] into [BufWriter]
    pub(crate) fn format<W: Write>(&self, w: &mut BufWriter<W>) -> Result<(), FormattingError> {
        writeln!(
            w,
            "{}",
            fmt_doris(
                &format!("{:<20}{}", self.serial_number, self.model),
                "ANT # / TYPE"
            )
        )?;

        if let Some(coords) = &self.approx_coordinates {
            writeln!(
                w,
                "{}",
                fmt_doris(
                    &format!("{:14.4}{:14.4}{:14.4}", coords.0, coords.1, coords.2),
                    "APPROX POSITION XYZ"
                )
            )?;
        }

        writeln!(
            w,
            "{}",
            fmt_doris(
                &format!(
                    "{:14.4}{:14.4}{:14.4}",
                    self.height.unwrap_or(0.0),
                    self.eastern.unwrap_or(0.0),
                    self.northern.unwrap_or(0.0)
                ),
                "ANTENNA: DELTA H/E/N"
            )
        )?;

        Ok(())
    }

    /// Sets desired model
    pub fn with_model(&self, m: &str) -> Self {
        let mut s = self.clone();
        s.model = m.to_string();
        s
    }

    /// Sets desired Serial Number
    pub fn with_serial_number(&self, serial_number: &str) -> Self {
        let mut s = self.clone();
        s.serial_number = serial_number.to_string();
        s
    }

    /// Sets reference/base coordinates (3D)
    pub fn with_base_coordinates(&self, coords: (f64, f64, f64)) -> Self {
        let mut s = self.clone();
        s.approx_coordinates = Some(coords);
        s
    }

    /// Sets antenna `h` eccentricity component
    pub fn with_height(&self, h: f64) -> Self {
        let mut s = self.clone();
        s.height = Some(h);
        s
    }

    /// Sets antenna `eastern` coordinates component
    pub fn with_eastern_component(&self, e: f64) -> Self {
        let mut s = self.clone();
        s.eastern = Some(e);
        s
    }

    /// Sets antenna `northern` coordiantes component
    pub fn with_northern_component(&self, n: f64) -> Self {
        let mut s = self.clone();
        s.northern = Some(n);
        s
    }
}
