use crate::{
    error::ParsingError,
    header::{Antenna, Header, Receiver, Version},
    observable::Observable,
    prelude::{Duration, Epoch, TimeScale, COSPAR, DOMES},
    station::GroundStation,
    Comments,
};

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

impl Header {
    /// Parse [Header] by consuming [BufReader] until end of this section
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParsingError> {
        let mut version = Version::default();

        let mut satellite = String::with_capacity(16);
        let mut program = Option::<String>::None;
        let mut run_by = Option::<String>::None;
        let mut date = Option::<String>::None;
        let mut observer = Option::<String>::None;
        let mut agency = Option::<String>::None;
        let mut license = Option::<String>::None;
        let mut doi = Option::<String>::None;
        let mut receiver = Option::<Receiver>::None;
        let mut antenna = Option::<Antenna>::None;
        let mut cospar = Option::<COSPAR>::None;
        let mut l1_l2_date_offset = Duration::default();
        let mut ground_stations = Vec::with_capacity(8);
        let mut scaling_factors = HashMap::new();
        let mut time_of_first_observation = Option::<Epoch>::None;
        let mut time_of_last_observation = Option::<Epoch>::None;

        let mut observables = Vec::<Observable>::with_capacity(8);
        let mut observables_continuation = false;

        let mut comments = Comments::default();

        for line in reader.lines() {
            if line.is_err() {
                continue;
            }

            let line = line.unwrap();

            if line.len() < 60 {
                continue; // --> invalid header content
            }

            let (content, marker) = line.split_at(60);

            let marker = marker.trim();

            if marker.eq("END OF HEADER") {
                // Special marker: done parsing
                break;
            }
            if marker.trim().eq("COMMENT") {
                // Comments are stored as is.
                comments.push(content.trim().to_string());
                continue;
            } else if marker.contains("RINEX VERSION / TYPE") {
                let (vers, rem) = line.split_at(20);
                let (type_str, rem) = rem.split_at(20);
                let (constell_str, _) = rem.split_at(20);

                let vers = vers.trim();
                let type_str = type_str.trim();
                let constell_str = constell_str.trim();

                if !type_str.eq("O") {
                    return Err(ParsingError::InvalidDoris);
                }

                if !constell_str.eq("D") {
                    return Err(ParsingError::InvalidDoris);
                }

                // version string
                version = Version::from_str(vers).or(Err(ParsingError::Version))?;
            } else if marker.contains("PGM / RUN BY / DATE") {
                let (pgm, rem) = line.split_at(20);
                let pgm = pgm.trim();
                if pgm.len() > 0 {
                    program = Some(pgm.to_string());
                }

                let (runby, rem) = rem.split_at(20);

                let runby = runby.trim();
                if runby.len() > 0 {
                    run_by = Some(runby.to_string());
                }

                let date_str = rem.split_at(20).0.trim();
                if date_str.len() > 0 {
                    date = Some(date_str.to_string());
                }
            } else if marker.contains("SATELLITE NAME") {
                let name = content.split_at(20).0.trim();
                satellite = name.to_string();
            } else if marker.contains("OBSERVER / AGENCY") {
                let (obs, ag) = content.split_at(20);
                let obs = obs.trim();
                let ag = ag.trim();

                if obs.len() > 0 {
                    observer = Some(obs.to_string());
                }

                if ag.len() > 0 {
                    agency = Some(ag.to_string());
                }
            } else if marker.contains("REC # / TYPE / VERS") {
                if let Ok(rx) = Receiver::from_str(content) {
                    receiver = Some(rx);
                }
            } else if marker.contains("SYS / SCALE FACTOR") {
                // // Parse scaling factor
                // let (factor, rem) = rem.split_at(6);
                // let factor = factor.trim();
                // let scaling = factor
                //     .parse::<u16>()
                //     .or(Err(ParsingError::SystemScalingFactor))?;

                // // parse end of line
                // let (_num, rem) = rem.split_at(3);
                // for observable_str in rem.split_ascii_whitespace() {
                //     let observable = Observable::from_str(observable_str)?;

                //     // latch scaling value
                //     if rinex_type == Type::DORIS {
                //         doris.with_scaling(observable, scaling);
                //     } else {
                //         observation.with_scaling(constell, observable, scaling);
                //     }
                // }
            } else if marker.contains("LICENSE OF USE") {
                let lic = content.split_at(40).0.trim();
                if lic.len() > 0 {
                    license = Some(lic.to_string());
                }
            } else if marker.contains("ANT # / TYPE") {
                let (sn, rem) = content.split_at(20);
                let (model, _) = rem.split_at(20);

                antenna = Some(
                    Antenna::default()
                        .with_model(model.trim())
                        .with_serial_number(sn.trim()),
                );
            } else if marker.contains("# OF STATIONS") {
            } else if marker.contains("TIME OF FIRST OBS") {
                time_of_first_observation = Some(Self::parse_time_of_obs(content)?);
            } else if marker.contains("TIME OF LAST OBS") {
                time_of_last_observation = Some(Self::parse_time_of_obs(content)?);
            } else if marker.contains("SYS / # / OBS TYPES") {
                // Self::parse_observables(content);
                observables_continuation = true;
            } else if marker.contains("COSPAR NUMBER") {
                cospar = Some(COSPAR::from_str(content.trim())?);
            } else if marker.contains("L2 / L1 DATE OFFSET") {
                // DORIS special case
                let content = content[1..].trim();

                let time_offset_us = content
                    .parse::<f64>()
                    .or(Err(ParsingError::DorisL1L2DateOffset))?;

                l1_l2_date_offset = Duration::from_microseconds(time_offset_us);
            } else if marker.contains("STATION REFERENCE") {
                // DORIS special case
                let station = GroundStation::from_str(content.trim())?;
                ground_stations.push(station);
            }
        }

        Ok(Header {
            version,
            comments,
            program,
            run_by,
            date,
            agency,
            observer,
            license,
            doi,
            receiver,
            antenna,
            cospar,
            satellite,
            scaling_factors,
            l1_l2_date_offset,
            ground_stations,
            time_of_first_observation,
            time_of_last_observation,
        })
    }

    fn parse_time_of_obs(content: &str) -> Result<Epoch, ParsingError> {
        let (_, rem) = content.split_at(2);
        let (y, rem) = rem.split_at(4);
        let (m, rem) = rem.split_at(6);
        let (d, rem) = rem.split_at(6);
        let (hh, rem) = rem.split_at(6);
        let (mm, rem) = rem.split_at(6);
        let (ss, rem) = rem.split_at(5);
        let (_dot, rem) = rem.split_at(1);
        let (ns, rem) = rem.split_at(8);

        // println!("Y \"{}\" M \"{}\" D \"{}\" HH \"{}\" MM \"{}\" SS \"{}\" NS \"{}\"", y, m, d, hh, mm, ss, ns); // DEBUG
        let mut y = y
            .trim()
            .parse::<u32>()
            .map_err(|_| ParsingError::EpochFormat)?;

        // handle OLD RINEX problem
        if y >= 79 && y <= 99 {
            y += 1900;
        } else if y < 79 {
            y += 2000;
        }

        let m = m
            .trim()
            .parse::<u8>()
            .map_err(|_| ParsingError::EpochFormat)?;

        let d = d
            .trim()
            .parse::<u8>()
            .map_err(|_| ParsingError::EpochFormat)?;

        let hh = hh
            .trim()
            .parse::<u8>()
            .map_err(|_| ParsingError::EpochFormat)?;

        let mm = mm
            .trim()
            .parse::<u8>()
            .map_err(|_| ParsingError::EpochFormat)?;

        let ss = ss
            .trim()
            .parse::<u8>()
            .map_err(|_| ParsingError::EpochFormat)?;

        let ns = ns
            .trim()
            .parse::<u32>()
            .map_err(|_| ParsingError::EpochFormat)?;

        /*
         * We set TAI as "default" Timescale.
         * Timescale might be omitted in Old RINEX formats,
         * In this case, we exit with "TAI" and handle that externally.
         */
        let mut ts = TimeScale::TAI;
        let rem = rem.trim();

        /*
         * Handles DORIS measurement special case,
         * offset from TAI, that we will convert back to TAI later
         */
        if !rem.is_empty() && rem != "DOR" {
            ts = TimeScale::from_str(rem.trim())?;
        }

        Epoch::from_str(&format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:08} {}",
            y, m, d, hh, mm, ss, ns, ts
        ))
        .map_err(|_| ParsingError::EpochFormat)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::{Epoch, Header};
    use std::str::FromStr;

    #[test]
    fn parse_time_of_obs() {
        let content = "  2021    12    21     0     0    0.0000000     GPS";
        let parsed = Header::parse_time_of_obs(&content).unwrap();
        assert_eq!(parsed, Epoch::from_str("2021-12-21T00:00:00 GPST").unwrap());

        let content = "  1995    01    01    00    00   00.000000             ";
        let parsed = Header::parse_time_of_obs(&content).unwrap();
        assert_eq!(parsed, Epoch::from_str("1995-01-01T00:00:00 TAI").unwrap());
    }
}
