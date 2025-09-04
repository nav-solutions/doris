use std::str::FromStr;

use crate::{prelude::*, tests::toolkit::*};

#[test]
fn v3_cs2rx18164() {
    let doris = DORIS::from_gzip_file("data/DOR/V3/cs2rx18164.gz").unwrap();

    assert_eq!(doris.header.version.major, 3);
    assert_eq!(doris.header.version.minor, 0);

    // Information about DORIS satellite (=measurement system)
    let cospar = COSPAR::from_str("2010-013A").unwrap();

    assert_eq!(doris.header.satellite, "CRYOSAT-2");
    assert_eq!(doris.header.cospar, Some(cospar));

    // Ground software (collecting and formatting)
    let agency = "CNES".to_string(); // Agency / producer
    let program = "Expert".to_string(); // Software name
    let run_by = "CNES".to_string(); // Operator
    let date = "20180614 090016 UTC".to_string(); // Date of production
    let observer = "SPA_BN1_4.7P1".to_string(); // Operator

    assert_eq!(doris.header.program, Some(program));
    assert_eq!(doris.header.run_by, Some(run_by));
    assert_eq!(doris.header.date, Some(date)); // currently not interpreted
    assert_eq!(doris.header.observer, Some(observer));
    assert_eq!(doris.header.agency, Some(agency));

    // Ground station hardware
    let receiver = Receiver::default()
        .with_firmware("1.00")
        .with_model("DGXX")
        .with_serial_number("CHAIN1");

    assert_eq!(doris.header.receiver, Some(receiver));

    // Measurements and physics to follow
    let observables = vec![
        Observable::UnambiguousPhaseRange(Frequency::DORIS1), // phase, in meters of prop.
        Observable::UnambiguousPhaseRange(Frequency::DORIS2),
        Observable::PseudoRange(Frequency::DORIS1), // decoded pseudo range
        Observable::PseudoRange(Frequency::DORIS2),
        Observable::Power(Frequency::DORIS1), // received power
        Observable::Power(Frequency::DORIS2), // received power
        Observable::FrequencyRatio,           // f1/f2 ratio (=drift image)
        Observable::Pressure,                 // pressure, at ground station level (hPa)
        Observable::Temperature,              // temperature, at ground station level (°C)
        Observable::HumidityRate,             // saturation rate, at ground station level (%)
    ];

    assert_eq!(doris.header.observables, observables);

    // Ground station hardware
    let antenna = Antenna::default()
        .with_model("STAREC")
        .with_serial_number("DORIS");

    assert_eq!(doris.header.antenna, Some(antenna));

    assert!(doris.header.doi.is_none());
    assert!(doris.header.license.is_none());

    let l1_l2_date_offset = Duration::from_microseconds(2.0);
    assert_eq!(doris.header.l1_l2_date_offset, l1_l2_date_offset);

    // 53 ground sites observed in this file
    assert_eq!(doris.header.ground_stations.len(), 53);

    // let time_of_first_obs = Epoch::from_str("2018-06-13T00:00:28.085331610 UTC")
    //    .unwrap();

    // assert_eq!(doris.header.time_of_first_observation, Some(time_of_first_obs));
    assert_eq!(doris.header.time_of_last_observation, None); // not specified by this header

    // We provide several search methods
    //  - DOMES site
    //  - Site name, label..
    //  - Site code (file depedent)..
    let domes = DOMES::from_str("10003S005").unwrap();

    let domes_matcher = Matcher::DOMES(domes);

    // example: retrieve site infos
    let toulouse = GroundStation::default()
        .with_domes(domes) // DOMES
        .with_site_label("TLSB") // site label
        .with_site_name("TOULOUSE") // site name
        .with_unique_id(13)
        .with_frequency_shift(0) // f1/f2 site shift for this file
        .with_beacon_revision(3); // DORIS 3rd generation

    assert_eq!(doris.ground_station(domes_matcher), Some(toulouse));

    // Standard file naming convention.
    // This could help when generating data from scratch.
    assert_eq!(doris.standard_filename(), "CS2RX18164.gz");

    let (l1, l2) = (
        Observable::from_str("L1").unwrap(),
        Observable::from_str("L2").unwrap(),
    );
    let (c1, c2) = (
        Observable::from_str("C1").unwrap(),
        Observable::from_str("C2").unwrap(),
    );
    let (w1, w2) = (
        Observable::from_str("W1").unwrap(),
        Observable::from_str("W2").unwrap(),
    );

    let f1f2 = Observable::from_str("F").unwrap();

    let press = Observable::from_str("P").unwrap();
    let temp = Observable::from_str("T").unwrap();
    let moist = Observable::from_str("H").unwrap();

    // testbench
    testbench(
        &doris,
        vec![
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:33.1799478 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::ClockOffset(ClockOffset::from_measured_offset(
                    Duration::from_seconds(-4.326631626),
                )),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:33.1799478 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: l1,
                        value: -677713.668,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:33.1799478 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: l2,
                        value: -133531.158,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:33.1799478 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: c1,
                        value: -139623093.08413,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:33.1799478 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: c2,
                        value: -139623340.44813,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:33.1799478 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: w1,
                        value: -128.150,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:33.1799478 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: w2,
                        value: -121.850,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:33.1799478 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: f1f2,
                        value: 169.370E-11,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:33.1799478 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: press,
                        value: 1003.702,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:33.1799478 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: temp,
                        value: 4.895,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:33.1799478 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: moist,
                        value: 81.602,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:36.179947800 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::ClockOffset(ClockOffset::from_measured_offset(
                    Duration::from_seconds(-4.326631626),
                )),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:36.179947800 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: l1,
                        value: -596018.152,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:36.179947800 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: w1,
                        value: -128.150,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:36.179947800 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: w2,
                        value: -121.850,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:00:36.179947800 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::StationObservation({
                    StationObservationData {
                        station: 1,
                        observable: moist,
                        value: 81.602,
                    }
                }),
            },
            TestPoint {
                epoch: Epoch::from_str("2018-06-13T00:02:26.179947800 TAI").unwrap(),
                flag: EpochFlag::OK,
                test_data: TestData::ClockOffset(ClockOffset::from_measured_offset(
                    Duration::from_seconds(-4.326631812),
                )),
            },
            // TestPoint {
            //     epoch: Epoch::from_str("2018-06-13T00:02:26.179947800 TAI").unwrap(),
            //     flag: EpochFlag::OK,
            //     test_data: TestData::StationObservation({
            //         StationObservationData {
            //             station: 2,
            //             observable: l1,
            //             value: -66483.813,
            //         }
            //     }),
            // },
            // TestPoint {
            //     epoch: Epoch::from_str("2018-06-13T00:02:26.179947800 TAI").unwrap(),
            //     flag: EpochFlag::OK,
            //     test_data: TestData::StationObservation({
            //         StationObservationData {
            //             station: 2,
            //             observable: l1,
            //             value: -13103.231,
            //         }
            //     }),
            // },
            // TestPoint {
            //     epoch: Epoch::from_str("2018-06-13T00:02:26.179947800 TAI").unwrap(),
            //     flag: EpochFlag::OK,
            //     test_data: TestData::StationObservation({
            //         StationObservationData {
            //             station: 2,
            //             observable: w1,
            //             value: -132.7,
            //         }
            //     }),
            // },
            // TestPoint {
            //     epoch: Epoch::from_str("2018-06-13T00:02:26.179947800 TAI").unwrap(),
            //     flag: EpochFlag::OK,
            //     test_data: TestData::StationObservation({
            //         StationObservationData {
            //             station: 2,
            //             observable: w2,
            //             value: -122.55,
            //         }
            //     }),
            // },
            // TestPoint {
            //     epoch: Epoch::from_str("2018-06-13T00:02:56.179947800 TAI").unwrap(),
            //     flag: EpochFlag::OK,
            //     test_data: TestData::ClockOffset(ClockOffset::from_measured_offset(
            //         Duration::from_seconds(-4.326632168),
            //     )),
            // },
            // TestPoint {
            //     epoch: Epoch::from_str("2018-06-13T00:02:26.179947800 TAI").unwrap(),
            //     flag: EpochFlag::OK,
            //     test_data: TestData::StationObservation({
            //         StationObservationData {
            //             station: 2,
            //             observable: l1,
            //             value: -1675820.378,
            //         }
            //     }),
            // },
            // TestPoint {
            //     epoch: Epoch::from_str("2018-06-13T00:02:26.179947800 TAI").unwrap(),
            //     flag: EpochFlag::OK,
            //     test_data: TestData::StationObservation({
            //         StationObservationData {
            //             station: 2,
            //             observable: l2,
            //             value: -330235.679,
            //         }
            //     }),
            // },
            // TestPoint {
            //     epoch: Epoch::from_str("2018-06-13T00:02:26.179947800 TAI").unwrap(),
            //     flag: EpochFlag::OK,
            //     test_data: TestData::StationObservation({
            //         StationObservationData {
            //             station: 3,
            //             observable: l1,
            //             value: -87906.919,
            //         }
            //     }),
            // },
            // TestPoint {
            //     epoch: Epoch::from_str("2018-06-13T00:02:26.179947800 TAI").unwrap(),
            //     flag: EpochFlag::OK,
            //     test_data: TestData::StationObservation({
            //         StationObservationData {
            //             station: 3,
            //             observable: l2,
            //             value: -17323.007,
            //         }
            //     }),
            // },
        ],
    );

    // basic test bench
    let residual = doris.substract(&doris);
    is_null_doris(&residual);

    // Easy to format new data
    doris.to_gzip_file("formatted.gz").unwrap();

    // parse back
    let parsed = DORIS::from_gzip_file("formatted.gz").unwrap_or_else(|e| {
        panic!("failed to parse 'formatted.gz' back: {}", e);
    });

    assert_eq!(parsed.header.satellite, "CRYOSAT-2");
    assert_eq!(parsed.standard_filename(), "CRYOS18164");

    // TODO testbench
}
