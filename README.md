DORIS
=====

[![Rust](https://github.com/nav-solutions/doris/actions/workflows/rust.yml/badge.svg)](https://github.com/nav-solutions/doris/actions/workflows/rust.yml)
[![Rust](https://github.com/nav-solutions/doris/actions/workflows/daily.yml/badge.svg)](https://github.com/nav-solutions/doris/actions/workflows/daily.yml)
[![crates.io](https://docs.rs/doris-rs/badge.svg)](https://docs.rs/doris-rs/)
[![crates.io](https://img.shields.io/crates/d/doris-rs.svg)](https://crates.io/crates/doris-rs)

[![MRSV](https://img.shields.io/badge/MSRV-1.82.0-orange?style=for-the-badge)](https://github.com/rust-lang/rust/releases/tag/1.82.0)
[![License](https://img.shields.io/badge/license-MPL_2.0-orange?style=for-the-badge&logo=mozilla)](https://github.com/nav-solutions/doris/blob/main/LICENSE)

`doris-rs` is dedicated to DORIS (special RINEX) files parsing, processing
and production.

Each DORIS file represents one measurement system, that means one satellite.
The DORIS format is a special RINEX Observation format. Unlike RINEX observations,
the measurement are performed spaceborn and consists in observing a network of ground stations.
DORIS is more complex than simple Observation RINEX, and also contains ground based information
at the time of the spaceborn observation (like temperature on the ground).

References:

- [RINEX format (Wikipedia)](https://en.wikipedia.org/wiki/RINEX) 
- DORIS format

NB: this parser is limited to the DORIS format exclusively, 
her RINEX like formats have their own parser:
  - [RINEX (obs, meteo, nav, clock)](https://github.com/nav-solutions/doris)
  - [IONEX (Ionosphere) Maps](https://github.com/nav-solutions/ionex)

NB: file formatting is work in progress and should be soon validated.

To contribute or join our community, you may:

- open an [Issue on Github.com](https://github.com/nav-solutions/doris/issues) 
- follow our [Discussions on Github.com](https://github.com/nav-solutions/discussions)
- join our [Discord channel](https://discord.gg/EqhEBXBmJh)

## Advantages :rocket: 

- Fast
- Seamless gzip compression support on `flate2` crate feature

## Inconvenients

- Epoch events are not really well supported at the moment (epoch flag >1).
This parser will store all observation data streams,
disregardning potential events in between (should not cause a panic).

## Citation and referencing

If you need to reference this work, please use the following model:

`Nav-solutions (2025), DORIS: analysis and processing (MPLv2), https://github.com/nav-solutions`

Contributions
=============

Contributions are welcomed, do not hesitate to open new issues
and submit Pull Requests through Github.

Getting Started
===============

```rust
use std::str::FromStr;
use doris_rs::prelude::*;
    
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

// example
doris.substract(&doris)
    .to_file("null-residuals.txt")
    .unwrap();

// Easy to format new data
doris.to_gzip_file("formatted.gz").unwrap();

// parse back
let parsed = DORIS::from_gzip_file("formatted.gz").unwrap_or_else(|e| {
    panic!("failed to parse 'formatted.gz' back: {}", e);
});

assert_eq!(parsed.header.satellite, "CRYOSAT-2");
assert_eq!(parsed.standard_filename(), "CRYOS18164");
```
