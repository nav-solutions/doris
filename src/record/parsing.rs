use std::io::{BufRead, BufReader, Read};

use crate::{
    epoch::parse_in_timescale as parse_epoch_in_timescale,
    error::ParsingError,
    prelude::{Comments, Duration, GroundStation, Header, Key, Matcher, Record, TimeScale},
};

// #[cfg(feature = "log")]
// use log::{error, debug};

impl Record {
    /// Parses the DORIS [Record] content by consuming the [Reader] until the end of stream.
    /// This requires reference to previously parsed [Header] section.
    pub fn parse<R: Read>(
        header: &mut Header,
        reader: &mut BufReader<R>,
    ) -> Result<Self, ParsingError> {
        const EPOCH_SIZE: usize = "YYYY MM DD HH MM SS.NNNNNNNNN  0".len();
        const CLOCK_OFFSET: usize = 38;
        const CLOCK_SIZE: usize = 14;
        const MIN_EPOCH_SIZE: usize = EPOCH_SIZE + CLOCK_SIZE + 2;

        // eos reached: process pending buffer & exit
        let mut eos = false;

        // current line storage
        let mut line_buf = String::with_capacity(128);

        // epoch storage
        let mut epoch_buf = String::with_capacity(1024);

        let mut comments = Comments::default();
        let mut record = Record::default();

        let mut obs_ptr = 0;
        let mut buf_len = 0;

        // Iterate and consume, one line at a time
        while let Ok(size) = reader.read_line(&mut line_buf) {
            if size == 0 {
                // reached EOS: consume buffer & exit
                eos |= true;
            }

            let line_len = line_buf.len();

            // println!("line buf \"{}\"", line_buf);

            if line_len > 60 {
                if line_buf.contains("COMMENT") {
                    // Comments are stored as is
                    let comment = line_buf.split_at(60).0.trim_end();
                    comments.push(comment.to_string());

                    line_buf.clear();
                    continue; // skip parsing
                }
            }

            // tries to assemble a complete epoch
            let mut new_epoch = false;

            // new epoch being detected or end of stream
            if buf_len > 0 {
                if line_buf.starts_with('>') || eos {
                    new_epoch = true;

                    // parse date & time
                    if buf_len < MIN_EPOCH_SIZE {
                        return Err(ParsingError::EpochFormat);
                    }

                    let epoch =
                        parse_epoch_in_timescale(&epoch_buf[2..2 + EPOCH_SIZE], TimeScale::TAI)?;

                    println!("epoch: {}", epoch);

                    // parse clock offset, if any
                    let clock_offset_secs = &epoch_buf[CLOCK_OFFSET..CLOCK_OFFSET + CLOCK_SIZE]
                        .trim()
                        .parse::<f64>()
                        .map_err(|_| ParsingError::ClockOffset)?;

                    let clock_offset = Duration::from_seconds(*clock_offset_secs);

                    // extrapolated clock ?
                    let mut clock_extrapolated = false;

                    if line_len > CLOCK_OFFSET + CLOCK_SIZE {
                        if line_buf[CLOCK_OFFSET + CLOCK_SIZE..].trim().eq("1") {
                            clock_extrapolated = true;
                        }
                    }

                    // station still unidentified
                    let mut station = Option::<&GroundStation>::None;

                    // continue parsing, identify and grab data
                    for line in epoch_buf.lines() {
                        println!("CONTENT: \"{}\"", line);

                        // new station
                        if line.starts_with("D") {
                            // station identification
                            let station_id = line[1..3]
                                .trim()
                                .parse::<u16>()
                                .map_err(|_| ParsingError::StationFormat)?;

                            let matcher = Matcher::ID(station_id);

                            // identification
                            if let Some(matching) = header
                                .ground_stations
                                .iter()
                                .filter(|station| station.matches(&matcher))
                                .reduce(|k, _| k)
                            {
                                println!("identified: {:?}", matching);
                                station = Some(matching);
                            } else {
                                #[cfg(feature = "logs")]
                                debug!("unidentified station: #{:02}", station_id);
                            }
                        }

                        if let Some(station) = station {
                            // correctly identified
                            let key = Key {
                                epoch,
                                station: station.clone(),
                            };
                        }
                    }

                    obs_ptr = 0;
                } // epoch parsing
            } // buf_len

            // clear on new epoch detection
            if new_epoch {
                buf_len = 0;
                epoch_buf.clear();
            }

            // always stack new content
            epoch_buf.push_str(&line_buf);
            buf_len += line_len;
            line_buf.clear(); // always clear newline buf

            if eos {
                break;
            }
        } //while

        Ok(record)
    }
}
