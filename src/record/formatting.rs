use crate::{
    epoch::format as format_epoch,
    error::FormattingError,
    prelude::{Epoch, EpochFlag, Header, Key, Record},
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
        // browse in chronological order
        for (epoch, flag) in self.epochs_iter() {
            writeln!(writer, "> {} 0 1", format_epoch(epoch))?;

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
                            for (key, value) in self.measurements.iter() {
                                writeln!(writer, "D{:02}", station.code)?;
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
