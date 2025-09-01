use std::io::{BufRead, BufReader, Read};

use crate::{
    epoch::parse_in_timescale as parse_epoch_in_timescale,
    error::ParsingError,
    prelude::{
        ClockOffset, Comments, Duration, Epoch, GroundStation, Header, Key, Matcher, Measurements,
        Observation, Record, TimeScale, SNR,
    },
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
        const OBSERVABLE_WIDTH: usize = 14;

        // eos reached: process pending buffer & exit
        let mut eos = false;

        // current line storage
        let mut buf_len = 0;
        let mut line_buf = String::with_capacity(128);

        // epoch storage
        let mut epoch_buf = String::with_capacity(1024);

        let mut record = Record::default();

        let mut obs_ptr = 0;
        let mut line_offset = 0;

        let observables = &header.observables;
        let nb_observables = observables.len();

        // Iterate and consume, one line at a time
        while let Ok(size) = reader.read_line(&mut line_buf) {
            if size == 0 {
                // reached EOS: consume buffer & exit
                eos |= true;
            }

            let line_len = line_buf.len();

            if line_len > 60 {
                if line_buf.contains("COMMENT") {
                    // Comments are stored as is
                    let comment = line_buf.split_at(60).0.trim_end();
                    record.comments.push(comment.to_string());

                    line_buf.clear();
                    continue; // skip parsing
                }
            }

            // tries to assemble a complete epoch
            let mut new_epoch = false;

            // new epoch
            if line_buf.starts_with('>') || eos {
                new_epoch = true;

                let mut obs_ptr = 0;
                let mut epoch = Epoch::default();
                let mut station = Option::<&GroundStation>::None;
                let mut clock_offset = Option::<ClockOffset>::None;

                for (nth, line) in epoch_buf.lines().enumerate() {
                    let line_len = line.len();

                    if nth == 0 {
                        // parse date & time
                        epoch = parse_epoch_in_timescale(&line[2..2 + EPOCH_SIZE], TimeScale::TAI)?;

                        println!("epoch: {}", epoch);

                        // parse clock offset, if any
                        let clock_offset_secs = &line[CLOCK_OFFSET..CLOCK_OFFSET + CLOCK_SIZE]
                            .trim()
                            .parse::<f64>()
                            .map_err(|_| ParsingError::ClockOffset)?;

                        let dt = Duration::from_seconds(*clock_offset_secs);
                        clock_offset = Some(ClockOffset::from_measured_offset(dt));

                        // clock extrapolation flag
                        if line_len > CLOCK_OFFSET + CLOCK_SIZE {
                            if line[CLOCK_OFFSET + CLOCK_SIZE..].trim().eq("1") {
                                if let Some(clock_offset) = &mut clock_offset {
                                    clock_offset.extrapolated = true;
                                }
                            }
                        }
                    } else {
                        if line.starts_with("D") {
                            // new station starting
                            obs_ptr = 0;

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
                                station = Some(matching);
                            } else {
                                #[cfg(feature = "logs")]
                                debug!("unidentified station: #{:02}", station_id);
                            }
                        }

                        // station must be identified
                        if let Some(station) = station {
                            println!("line={} station={:?}", nth, station);

                            // identified
                            let key = Key {
                                epoch,
                                station: station.clone(),
                            };

                            let mut offset = 3;

                            loop {
                                println!("obs_ptr={}", obs_ptr);

                                if offset + OBSERVABLE_WIDTH + 1 < line_len {
                                    let slice = &line[offset..offset + OBSERVABLE_WIDTH];
                                    println!("slice \"{}\"", slice);

                                    match slice.trim().parse::<f64>() {
                                        Ok(value) => {
                                            let mut observation = Observation::default();

                                            if let Some(measurements) =
                                                record.measurements.get_mut(&key)
                                            {
                                                measurements.add_observation(
                                                    observables[obs_ptr],
                                                    observation,
                                                );
                                            } else {
                                                let mut measurements = Measurements::default();
                                                measurements.add_observation(
                                                    observables[obs_ptr],
                                                    observation,
                                                );

                                                measurements.satellite_clock_offset = clock_offset;

                                                record
                                                    .measurements
                                                    .insert(key.clone(), measurements);
                                            }
                                        },
                                        Err(e) => {
                                            println!("observation parsing error: {}", e);
                                        },
                                    }
                                }

                                offset += OBSERVABLE_WIDTH;

                                if offset + 1 < line_len {
                                    let slice = &line[offset..offset + 1];
                                    // println!("slice \"{}\"", slice);

                                    if let Ok(snr) = slice.trim().parse::<SNR>() {
                                        if let Some(measurements) =
                                            record.measurements.get_mut(&key)
                                        {
                                            if let Some(observation) = measurements
                                                .observations
                                                .get_mut(&observables[obs_ptr])
                                            {
                                                observation.snr = Some(snr);
                                            }
                                        }
                                    }
                                }

                                offset += 1;

                                if offset + 1 < line_len {
                                    let slice = &line[offset..offset + 1];
                                    // println!("slice \"{}\"", slice);

                                    // if let Ok(flag) = slice.trim().parse::<Flag>() {
                                    //     if let Some(measurements) =
                                    //         record.measurements.get_mut(&key)
                                    //     {
                                    //         if let Some(observation) = measurements
                                    //             .observations
                                    //             .get_mut(&observables[obs_ptr])
                                    //         {
                                    //             observation.phase_flag = Some(flag);
                                    //         }
                                    //     }
                                    // }
                                }

                                offset += 1;
                                obs_ptr += 1;

                                if offset >= line_len {
                                    break;
                                }

                                // detect potential errors
                                if obs_ptr >= nb_observables {
                                    break;
                                }
                            }
                        }
                    }
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
