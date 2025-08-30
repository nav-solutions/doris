use crate::prelude::*;
use std::str::FromStr;

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

    // Example: retrieve site observation

    // API makes it easy to format new data
    doris.to_gzip_file("formatted.gz").unwrap();

    // parse back
    let parsed = DORIS::from_gzip_file("formatted.gz").unwrap();

    // testbench
    // assert_eq!(parsed, doris);
}
