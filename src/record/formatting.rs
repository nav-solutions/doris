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
        for (key, measurements) in self.measurements.iter() {
            let (year, month, day, hours, mins, secs, nanos) =
                key.epoch.to_gregorian(key.epoch.time_scale);

            write!(
                writer,
                "> {:04} {:02} {:02} {:02} {:02} {:02}.{:09}  {}",
                year, month, day, hours, mins, secs, nanos, key.flag
            )?;

            // number of station at this epoch
            let num_stations = measurements
                .observations
                .keys()
                .map(|k| k.station.code)
                .unique()
                .count();
            write!(writer, "{:3}", num_stations)?;

            // conclude line with clock offset
            if let Some(clock_offset) = measurements.satellite_clock_offset {
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
                    // browse by station ID#
                    for (nth_station, station) in header.ground_stations.iter().enumerate() {
                        write!(writer, "D{:02}", station.code)?;

                        // following header specs
                        for (nth_observable, observable) in header.observables.iter().enumerate() {
                            let obs_key = ObservationKey {
                                observable: *observable,
                                station: station.clone(),
                            };

                            if let Some(observation) = measurements.observations.get(&obs_key) {
                                write!(writer, "{:14.3}  ", observation.value)?;
                            } else {
                                // BLANK
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
                    // TODO not supported yet
                },
            }
        }

        Ok(())
    }
}
