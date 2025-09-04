use crate::{fmt_comment, fmt_doris, header::Header, prelude::FormattingError};

use std::{
    io::{BufWriter, Write},
    str::FromStr,
};

use hifitime::efmt::{Format, Formatter};

impl Header {
    /// Formats [Header] into [Write]able interface, using efficient buffering.
    pub fn format<W: Write>(&self, w: &mut BufWriter<W>) -> Result<(), FormattingError> {
        writeln!(
            w,
            "{}",
            fmt_doris(
                &format!(
                    "{:6}.{:02}           O                   D",
                    self.version.major, self.version.minor
                ),
                "RINEX VERSION / TYPE"
            )
        )?;

        writeln!(w, "{}", fmt_doris(&self.satellite, "SATELLITE NAME"))?;

        if let Some(cospar) = &self.cospar {
            writeln!(w, "{}", fmt_doris(&cospar.to_string(), "COSPAR"))?;
        }

        self.format_prog_runby(w)?;
        self.format_observer_agency(w)?;

        let mut string = format!("D {:10}", self.observables.len());

        for observable in self.observables.iter() {
            string.push_str(&format!(" {:x}", observable));
        }

        writeln!(w, "{}", fmt_doris(&string, "SYS / # / OBS TYPES"))?;

        if let Some(epoch) = self.time_of_first_observation {
            let (year, month, day, hours, mins, secs, nanos) = epoch.to_gregorian(epoch.time_scale);
            writeln!(
                w,
                "{}",
                fmt_doris(
                    &format!(
                        "{:6} {:5} {:5} {:5} {:5} {:4}.{}    DOR",
                        year, month, day, hours, mins, secs, nanos
                    ),
                    "TIME OF FIRST OBS"
                )
            )?;
        }

        if let Some(epoch) = self.time_of_last_observation {
            let (year, month, day, hours, mins, secs, nanos) = epoch.to_gregorian(epoch.time_scale);
            writeln!(
                w,
                "{}",
                fmt_doris(
                    &format!(
                        "{:6} {:5} {:5} {:5} {:5} {:4}.{}    DOR",
                        year, month, day, hours, mins, secs, nanos
                    ),
                    "TIME OF LAST OBS"
                )
            )?;
        }

        writeln!(
            w,
            "{}",
            fmt_doris(
                &format!("{:6}", self.ground_stations.len()),
                "# OF STATIONS"
            )
        )?;

        for station in self.ground_stations.iter() {
            writeln!(
                w,
                "{}",
                fmt_doris(&format!("{:x}", station), "STATION REFERENCE")
            )?;
        }

        writeln!(w, "{}", fmt_doris("", "END OF HEADER"))?;
        Ok(())
    }

    /// Formats "PGM / RUN BY / DATE"
    fn format_prog_runby<W: Write>(&self, w: &mut BufWriter<W>) -> Result<(), FormattingError> {
        let program = format!(
            "doris-rs v{}",
            Self::format_pkg_version(env!("CARGO_PKG_VERSION"))
        );

        let mut string = format!("{:<20}", program);

        if let Some(runby) = &self.run_by {
            let formatted = format!("{:<20}", runby);
            string.push_str(&formatted);
        } else {
            string.push_str("                    ");
        };

        if let Some(date) = &self.date {
            string.push_str(date);
        } else {
            string.push_str("                    ");
        };

        // PGM / RUN BY / DATE
        writeln!(w, "{}", fmt_doris(&string, "PGM / RUN BY / DATE"),)?;

        Ok(())
    }

    /// Formats "OBSERVER / AGENCY"
    fn format_observer_agency<W: Write>(
        &self,
        w: &mut BufWriter<W>,
    ) -> Result<(), FormattingError> {
        let mut string = if let Some(observer) = &self.observer {
            format!("{:<20}", observer)
        } else {
            "                    ".to_string()
        };

        if let Some(agency) = &self.agency {
            string.push_str(agency);
        } else {
            string.push_str("                    ");
        };

        writeln!(w, "{}", fmt_doris(&string, "OBSERVER / AGENCY"),)?;

        Ok(())
    }

    /// Formats all comments
    fn format_comments<W: Write>(&self, w: &mut BufWriter<W>) -> Result<(), FormattingError> {
        for comment in self.comments.iter() {
            writeln!(w, "{}", fmt_comment(comment))?;
        }
        Ok(())
    }
}
