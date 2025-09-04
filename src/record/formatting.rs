use crate::{
    error::FormattingError,
    prelude::{Epoch, EpochFlag, GroundStation, Header, Key, ObservationKey, Record},
};

use itertools::Itertools;

use std::io::{BufWriter, Write};

impl Record {
    /// Format this DORIS [Record] according to the standard specifications,
    /// into [W]ritable interface.
    pub fn format<W: Write>(
        &self,
        writer: &mut BufWriter<W>,
        header: &Header,
    ) -> Result<(), FormattingError> {
        let num_observables = header.observables.len();

        // browse in chronological order
        for (key, measurement) in self.measurements.iter() {
            let (year, month, day, hours, mins, secs, nanos) =
                key.epoch.to_gregorian(key.epoch.time_scale);

            write!(
                writer,
                "> {:04} {:02} {:02} {:02} {:02} {:02}.{:09}  {}",
                year, month, day, hours, mins, secs, nanos, key.flag
            )?;

            // number of station at this epoch
            let num_stations = measurement
                .observations
                .keys()
                .map(|k| k.station.code)
                .unique()
                .count();

            write!(writer, "{:3}", num_stations)?;

            // conclude line with clock offset
            if let Some(clock_offset) = measurement.satellite_clock_offset {
                write!(
                    writer,
                    "       {:.9} {}\n",
                    clock_offset.offset.to_seconds(),
                    clock_offset.extrapolated as u8
                )?;
            } else {
                write!(writer, "\n")?;
            }

            match key.flag {
                EpochFlag::OK | EpochFlag::PowerFailure => {
                    for station_id in measurement
                        .observations
                        .keys()
                        .map(|k| k.station.code)
                        .unique()
                        .sorted()
                    {
                        write!(writer, "D{:02}", station_id)?;

                        // following header specs
                        for (nth_observable, observable) in header.observables.iter().enumerate() {
                            if let Some(observation) = measurement
                                .observations
                                .iter()
                                .filter_map(|(k, v)| {
                                    if k.station.code == station_id && k.observable == *observable {
                                        Some(v)
                                    } else {
                                        None
                                    }
                                })
                                .reduce(|k, _| k)
                            {
                                write!(writer, "{:14.3}  ", observation.value)?;
                            } else {
                                write!(writer, "                  ")?;
                            }

                            if nth_observable == num_observables - 1 {
                                write!(writer, "\n")?;
                            } else {
                                if (nth_observable % 5) == 4 {
                                    write!(writer, "\n   ")?;
                                }
                            }
                        }
                    }
                },
                todo => {
                    // TODO: events: not supported yet
                },
            }
        }

        Ok(())
    }
}
