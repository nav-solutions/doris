DORIS
=====

[![Rust](https://github.com/nav-solutions/doris/actions/workflows/rust.yml/badge.svg)](https://github.com/nav-solutions/doris/actions/workflows/rust.yml)
[![Rust](https://github.com/nav-solutions/doris/actions/workflows/daily.yml/badge.svg)](https://github.com/nav-solutions/doris/actions/workflows/daily.yml)
[![crates.io](https://docs.rs/doris/badge.svg)](https://docs.rs/doris/)
[![crates.io](https://img.shields.io/crates/d/doris.svg)](https://crates.io/crates/doris)

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


To contribute or join our community, you may:

- open an [Issue on Github.com](https://github.com/nav-solutions/doris/issues) 
- follow our [Discussions on Github.com](https://github.com/nav-solutions/discussions)
- join our [Discord channel](https://discord.gg/EqhEBXBmJh)

## Advantages :rocket: 

- Fast
- Seamless gzip compression support on `flate2` crate feature
- File formatting

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
