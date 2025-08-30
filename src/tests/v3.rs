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
    let agency = "CNES".to_string();
    let program = "Expert".to_string();
    let run_by = "CNES".to_string();
    let date = "20180614 090016 UTC".to_string();
    let observer = "SPA_BN1_4.7P1".to_string();

    assert_eq!(doris.header.program, Some(program));
    assert_eq!(doris.header.run_by, Some(run_by));
    assert_eq!(doris.header.date, Some(date));
    assert_eq!(doris.header.observer, Some(observer));
    assert_eq!(doris.header.agency, Some(agency));

    // Ground station hardware info
    let receiver = Receiver::default()
        .with_firmware("1.00")
        .with_model("DGXX")
        .with_serial_number("CHAIN1");

    assert_eq!(doris.header.receiver, Some(receiver));

    // Ground station hardware info
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
}
