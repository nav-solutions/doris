use crate::{
    header::{Antenna, Receiver, Version},
    prelude::{COSPAR, DOMES},
    station::Groundstation,
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

        let mut program = Option::<String>::None;
        let mut run_by = Option::<String>::None;
        let mut date = Option::<String>::None;
        let mut observer = Option::<String>::None;
        let mut agency = Option::<String>::None;
        let mut license = Option::<String>::None;
        let mut doi = Option::<String>::None;
        let mut receiver = Option::<Receiver>::None;
        let mut antenna = Option::<Antenna>::None;

        let mut observables = Vec::<Observable>::with_capacity(8);
        let mut observable_continuation = false;

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

                let type_str = type_str.trim();
                let constell_str = constell_str.trim();

                // File type identification
                if type_str == "O" && constell_str == "D" {
                    rinex_type = Type::DORIS;
                } else {
                    rinex_type = Type::from_str(type_str)?;
                }

                // Determine (file) Constellation
                //  1. NAV SPECIAL CASE
                //  2. OTHER
                match rinex_type {
                    Type::NavigationData => {
                        if type_str.contains("GLONASS") {
                            // old GLONASS NAV : no constellation field
                            constellation = Some(Constellation::Glonass);
                        } else if type_str.contains("GPS NAV DATA") {
                            constellation = Some(Constellation::GPS);
                        } else if type_str.contains("IRNSS NAV DATA") {
                            constellation = Some(Constellation::IRNSS);
                        } else if type_str.contains("GNSS NAV DATA") {
                            constellation = Some(Constellation::Mixed);
                        } else if type_str.eq("NAVIGATION DATA") {
                            if constell_str.is_empty() {
                                // old GPS NAVIGATION DATA
                                constellation = Some(Constellation::GPS);
                            } else {
                                // Modern NAVIGATION DATA
                                if let Ok(c) = Constellation::from_str(constell_str) {
                                    constellation = Some(c);
                                }
                            }
                        }
                    },
                    Type::MeteoData | Type::DORIS => {
                        // no constellation associated to them
                    },
                    _ => {
                        // any other
                        // regular files
                        if let Ok(c) = Constellation::from_str(constell_str) {
                            constellation = Some(c);
                        }
                    },
                }
                /*
                 * Parse version descriptor
                 */
                let vers = vers.trim();
                version = Version::from_str(vers).or(Err(ParsingError::VersionParsing))?;
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
            } else if marker.contains("STATION INFORMATION") {
                let url = content.split_at(40).0.trim();
                if url.len() > 0 {
                    station_url = Some(url.to_string());
                }
            } else if marker.contains("LICENSE OF USE") {
                let lic = content.split_at(40).0.trim();
                if lic.len() > 0 {
                    license = Some(lic.to_string());
                }
            } else if marker.contains("CENTER OF MASS: XYZ") {
                // TODO
            } else if marker.contains("APPROX POSITION XYZ") {
                let mut num_items = 0;
                let (mut x_ecef_m, mut y_ecef_m, mut z_ecef_m) = (0.0_f64, 0.0_f64, 0.0_f64);

                for (nth, item) in content.split_ascii_whitespace().enumerate() {
                    if let Ok(ecef_m) = item.trim().parse::<f64>() {
                        match nth {
                            0 => {
                                x_ecef_m = ecef_m;
                            },
                            1 => {
                                y_ecef_m = ecef_m;
                            },
                            2 => {
                                num_items = 3;
                                z_ecef_m = ecef_m;
                            },
                            _ => {},
                        }
                    }
                }

                if num_items == 3 {
                    rx_position = Some((x_ecef_m, y_ecef_m, z_ecef_m));
                }
            } else if marker.contains("ANT # / TYPE") {
                let (sn, rem) = content.split_at(20);
                let (model, _) = rem.split_at(20);

                rcvr_antenna = Some(
                    Antenna::default()
                        .with_model(model.trim())
                        .with_serial_number(sn.trim()),
                );
            } else if marker.contains("# OF STATIONS") {
            } else if marker.contains("TIME OF FIRST OBS") {
                let time_of_first_obs = Self::parse_time_of_obs(content)?;
                timeof_first_obs = Some(time_of_first_obs);
            } else if marker.contains("TIME OF LAST OBS") {
                let time_of_last_obs = Self::parse_time_of_obs(content)?;
                timeof_last_obs = Some(time_of_last_obs);
            } else if marker.contains("TYPES OF OBS") {
                // these observations can serve both Observation & Meteo RINEX
                Self::parse_v2_observables(content, constellation, &mut meteo, &mut observation);
            } else if marker.contains("SYS / # / OBS TYPES") {
                Self::parse_doris_observables(content, &mut doris);
                observables_continuation = true;
            } else if marker.contains("COSPAR NUMBER") {
                cospar = Some(COSPAR::from_str(content.trim())?);
            } else if marker.contains("L2 / L1 DATE OFFSET") {
                // DORIS special case
                let content = content[1..].trim();

                let time_offset_us = content
                    .parse::<f64>()
                    .or(Err(ParsingError::DorisL1L2DateOffset))?;

                doris.u2_s1_time_offset = Duration::from_microseconds(time_offset_us);
            } else if marker.contains("STATION REFERENCE") {
                // DORIS special case
                let station = DorisStation::from_str(content.trim())?;
                doris.stations.push(station);
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
            station_url,
            receiver,
            antenna,
            cospar,
            stations,
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
            .map_err(|_| ParsingError::DatetimeParsing)?;

        // handle OLD RINEX problem
        if y >= 79 && y <= 99 {
            y += 1900;
        } else if y < 79 {
            y += 2000;
        }

        let m = m
            .trim()
            .parse::<u8>()
            .map_err(|_| ParsingError::DatetimeParsing)?;

        let d = d
            .trim()
            .parse::<u8>()
            .map_err(|_| ParsingError::DatetimeParsing)?;

        let hh = hh
            .trim()
            .parse::<u8>()
            .map_err(|_| ParsingError::DatetimeParsing)?;

        let mm = mm
            .trim()
            .parse::<u8>()
            .map_err(|_| ParsingError::DatetimeParsing)?;

        let ss = ss
            .trim()
            .parse::<u8>()
            .map_err(|_| ParsingError::DatetimeParsing)?;

        let ns = ns
            .trim()
            .parse::<u32>()
            .map_err(|_| ParsingError::DatetimeParsing)?;

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
        .map_err(|_| ParsingError::DatetimeParsing)
    }

    /*
     * Parse IONEX grid
     */
    fn parse_grid(line: &str) -> Result<Linspace, ParsingError> {
        let mut start = 0.0_f64;
        let mut end = 0.0_f64;
        let mut spacing = 0.0_f64;
        for (index, item) in line.split_ascii_whitespace().enumerate() {
            let item = item.trim();
            match index {
                0 => {
                    start = f64::from_str(item).or(Err(ParsingError::IonexGridSpecs))?;
                },
                1 => {
                    end = f64::from_str(item).or(Err(ParsingError::IonexGridSpecs))?;
                },
                2 => {
                    spacing = f64::from_str(item).or(Err(ParsingError::IonexGridSpecs))?;
                },
                _ => {},
            }
        }
        if spacing == 0.0 {
            // avoid linspace verification in this case
            Ok(Linspace {
                start,
                end,
                spacing,
            })
        } else {
            let grid = Linspace::new(start, end, spacing)?;
            Ok(grid)
        }
    }

    /// Parse list of [Observable]s which applies to both METEO and OBS RINEX
    pub(crate) fn parse_v2_observables(
        line: &str,
        constell: Option<Constellation>,
        meteo: &mut MeteoHeader,
        observation: &mut ObservationHeader,
    ) {
        lazy_static! {
            /*
             *  We support GPS, Glonass, Galileo, SBAS and BDS as per v2.11.
             */
            static ref KNOWN_V2_CONSTELLS: [Constellation; 5] = [
                Constellation::GPS,
                Constellation::SBAS,
                Constellation::Glonass,
                Constellation::Galileo,
                Constellation::BeiDou,
            ];
        }
        let line = line.split_at(6).1;
        for item in line.split_ascii_whitespace() {
            if let Ok(obs) = Observable::from_str(item.trim()) {
                match constell {
                    Some(Constellation::Mixed) => {
                        for constell in KNOWN_V2_CONSTELLS.iter() {
                            if let Some(codes) = observation.codes.get_mut(constell) {
                                codes.push(obs.clone());
                            } else {
                                observation.codes.insert(*constell, vec![obs.clone()]);
                            }
                        }
                    },
                    Some(c) => {
                        if let Some(codes) = observation.codes.get_mut(&c) {
                            codes.push(obs.clone());
                        } else {
                            observation.codes.insert(c, vec![obs.clone()]);
                        }
                    },
                    None => meteo.codes.push(obs),
                }
            }
        }
    }

    /// Parse list of [Observable]s which applies to both METEO and OBS RINEX
    fn parse_v3_observables(
        line: &str,
        current_constell: &mut Option<Constellation>,
        observation: &mut ObservationHeader,
    ) {
        let (possible_counter, items) = line.split_at(6);
        if !possible_counter.is_empty() {
            let code = &possible_counter[..1];
            if let Ok(c) = Constellation::from_str(code) {
                *current_constell = Some(c);
            }
        }
        if let Some(constell) = current_constell {
            // system correctly identified
            for item in items.split_ascii_whitespace() {
                if let Ok(observable) = Observable::from_str(item) {
                    if let Some(codes) = observation.codes.get_mut(constell) {
                        codes.push(observable);
                    } else {
                        observation.codes.insert(*constell, vec![observable]);
                    }
                }
            }
        }
    }
    /*
     * Parse list of DORIS observables
     */
    fn parse_doris_observables(line: &str, doris: &mut DorisHeader) {
        let items = line.split_at(6).1;
        for item in items.split_ascii_whitespace() {
            if let Ok(observable) = Observable::from_str(item) {
                doris.observables.push(observable);
            }
        }
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
