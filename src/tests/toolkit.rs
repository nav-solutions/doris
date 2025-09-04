use crate::prelude::{
    ClockOffset, Epoch, EpochFlag, Key, Matcher, Observable, Observation, ObservationKey, DORIS,
};

#[derive(Debug)]
pub struct StationObservationData {
    pub station: u16,
    pub value: f64,
    pub observable: Observable,
}

#[derive(Debug)]
pub enum TestData {
    ClockOffset(ClockOffset),
    StationObservation(StationObservationData),
}

pub struct TestPoint {
    pub epoch: Epoch,
    pub flag: EpochFlag,
    pub test_data: TestData,
}

impl TestPoint {
    pub fn to_record_key(&self) -> Key {
        Key {
            epoch: self.epoch,
            flag: self.flag,
        }
    }
}

// /// Runs strict equality comparison, panics on any failure
// pub fn doris_comparison(dut: &DORIS, model: &DORIS) {
// }

/// Tests all data points in this [DORIS] record
pub fn testbench(dut: &DORIS, testpoints: Vec<TestPoint>) {
    for testpoint in testpoints.iter() {
        let key = testpoint.to_record_key();

        // locate measurement
        let measurements = dut.record.measurements.get(&key).unwrap_or_else(|| {
            panic!("Missing measurement @ {:?}", key);
        });

        match &testpoint.test_data {
            TestData::ClockOffset(clock_offset) => {
                let sat_offset = measurements.satellite_clock_offset.unwrap_or_else(|| {
                    panic!("Unreported satellite clock offset @ {}", key.epoch);
                });

                assert_eq!(
                    sat_offset.extrapolated, clock_offset.extrapolated,
                    "incorrect clock offset interpretation @ {}",
                    key.epoch,
                );

                let error = (sat_offset.offset.total_nanoseconds()
                    - clock_offset.offset.total_nanoseconds())
                .abs();

                assert!(
                    error < 1,
                    "invalid clock offset reported @ {} (offset={} err={}ns)",
                    key.epoch,
                    sat_offset.offset,
                    error
                );
            },
            TestData::StationObservation(station_data) => {
                // match this station
                let station = dut
                    .header
                    .ground_station(station_data.station)
                    .unwrap_or_else(|| {
                        panic!(
                            "Station id={} not found at {}",
                            key.epoch, station_data.station
                        );
                    });

                // locate
                let obs_key = ObservationKey {
                    observable: station_data.observable,
                    station: station,
                };

                let observation = measurements.observations.get(&obs_key).unwrap_or_else(|| {
                    panic!(
                        "{} missing {} measurement for station D{:02}",
                        key.epoch, station_data.observable, station_data.station
                    );
                });

                // TODO (SNR)
                // assert_eq!(observation.snr, station_data;

                let error = (observation.value - station_data.value).abs();
                assert!(
                    error < 1.0E-3,
                    "invalid D{:02} {} measurement @ {} (value={}, error={})",
                    station_data.station,
                    key.epoch,
                    station_data.observable,
                    observation.value,
                    error
                );
            },
        }
    }
}
