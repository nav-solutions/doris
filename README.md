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

The DORIS format is a special RINEX Observation format. Unlike RINEX observations,
the measurement is airborne and consists in observing a network of ground stations.
Each DORIS file represents one measurement system, that means one satellite.

References:

- [RINEX format (Wikipedia)](https://en.wikipedia.org/wiki/RINEX) 
- DORIS format

To contribute to either of our project or join our community, you way
- open an [Issue on Github.com](https://github.com/nav-solutions/doris/issues) 
- follow our [Discussions on Github.com](https://github.com/nav-solutions/discussions)
- join our [Discord channel](https://discord.gg/EqhEBXBmJh)

## Advantages :rocket: 

- Fast
- Limited to DORIS exclusively, 
other RINEX-like formats have their own parser:
  - [RINEX (obs, meteo, nav, clock)](https://github.com/nav-solutions/doris)
  - [IONEX maps](https://github.com/nav-solutions/ionex)

## Citation and referencing

If you need to reference this work, please use the following model:

`Nav-solutions (2025), DORIS: analysis and processing (MPLv2), https://github.com/nav-solutions`

Contributions
=============

Contributions are welcomed, do not hesitate to open new issues
and submit Pull Requests through Github.
