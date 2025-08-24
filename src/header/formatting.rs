//! RINEX header formatting

use crate::{
    fmt_comment, fmt_doris,
    header::Header,
    prelude::{Constellation, FormattingError},
};

use std::io::{BufWriter, Write};

impl Header {
    /// Formats [Header] into [Write]able interface, using efficient buffering.
    pub fn format<W: Write>(&self, w: &mut BufWriter<W>) -> Result<(), FormattingError> {
        self.format_prog_runby(w)?;
        self.format_observer_agency(w)?;
        self.format_sampling_interval(w)?;

        writeln!(w, "{}", fmt_doris("", "END OF HEADER"))?;
        Ok(())
    }

    /// Formats "PGM / RUN BY / DATE"
    fn format_prog_runby<W: Write>(&self, w: &mut BufWriter<W>) -> Result<(), FormattingError> {
        let mut string = if let Some(program) = &self.program {
            format!("{:<20}", program)
        } else {
            "                    ".to_string()
        };

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
        writeln!(w, "{}", fmt_rinex(&string, "PGM / RUN BY / DATE"),)?;

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

        writeln!(w, "{}", fmt_rinex(&string, "OBSERVER / AGENCY"),)?;

        Ok(())
    }

    /// Formats "INTERVAL"
    fn format_sampling_interval<W: Write>(
        &self,
        w: &mut BufWriter<W>,
    ) -> Result<(), FormattingError> {
        if let Some(interval) = &self.sampling_interval {
            writeln!(
                w,
                "{}",
                fmt_rinex(&format!("{:6}", interval.to_seconds()), "INTERVAL")
            )?;
        }
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
