use crate::{
    epoch::format as format_epoch,
    error::FormattingError,
    prelude::{Epoch, EpochFlag, GroundStation, Header, Key, Record},
};

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
        for (epoch, flag) in self.epochs_iter() {
            write!(writer, "> {}00  {}", format_epoch(epoch), flag)?;

            // determine number of station at this epoch
            let mut num_stations = 0;
            let mut prev_code = 0;

            for station in header.ground_stations.iter() {
                let key = Key {
                    epoch,
                    flag,
                    station: station.clone(),
                };

                if self.measurements.get(&key).is_some() {
                    num_stations += 1;
                }

                prev_code = station.code;
            }

            write!(writer, "{:3}", num_stations)?;

            match flag {
                EpochFlag::OK | EpochFlag::PowerFailure => {
                    // browse by station ID#
                    for station in header.ground_stations.iter() {
                        let key = Key {
                            epoch,
                            flag,
                            station: station.clone(),
                        };

                        if let Some(measurements) = self.measurements.get(&key) {
                            // conclude with clock offset (if any)
                            if let Some(clock_offset) = measurements.satellite_clock_offset {
                                write!(
                                    writer,
                                    "{:14.3} {}\n",
                                    clock_offset.offset.to_seconds(),
                                    clock_offset.extrapolated as u8
                                )?;
                            } else {
                                write!(writer, "\n")?;
                            }

                            // browse by observables specs
                            for (nth_observable, hd_observable) in
                                header.observables.iter().enumerate()
                            {
                                if nth_observable == 0 {
                                    write!(writer, "D{:02}", station.code)?;
                                }

                                if let Some(observation) = measurements
                                    .observations
                                    .iter()
                                    .filter_map(|(observable, observation)| {
                                        if observable == hd_observable {
                                            Some(observation)
                                        } else {
                                            None
                                        }
                                    })
                                    .reduce(|k, _| k)
                                {
                                    write!(writer, "{:14.3}  ", observation.value)?;

                                    if nth_observable == num_observables - 1 {
                                        write!(writer, "\n")?;
                                    } else {
                                        if (nth_observable % 5) == 4 {
                                            write!(writer, "\n")?;
                                        }
                                    }
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
