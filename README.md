DORIS
=====

`doris-rs` is dedicated to DORIS (special RINEX) files parsing, processing
and production.

The DORIS format is a special RINEX Observation format. Unlike RINEX observations,
the measurement is realized in space and consists in observing a network of ground stations.
Each DORIS file represents one measurement system, that means one satellite.

While RINEX observations are limited to 10E-9 precision, DORIS is limited to 10E-12.
