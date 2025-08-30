use crate::prelude::{ClockOffset, Epoch, Key, Matcher, Observable, Observation, DORIS};

#[derive(Debug)]
pub enum Measurement {
    ClockOffset(ClockOffset),
    Observation((Observable, Observation)),
}

pub struct TestPoint {
    pub epoch: Epoch,
    pub station_id: u16,
    pub measurements: Vec<Measurement>,
}

impl TestPoint {
    pub fn to_record_key(&self, dut: &DORIS) -> Key {
        let station = dut
            .header
            .ground_station(self.station_id)
            .unwrap_or_else(|| {
                panic!("Ground station #{:02} does not exist", self.station_id);
            });

        Key {
            station,
            epoch: self.epoch,
        }
    }
}

// /// Runs strict equality comparison, panics on any failure
// pub fn doris_comparison(dut: &DORIS, model: &DORIS) {
// }

/// Tests all data points in this [DORIS] record
pub fn testbench(dut: &DORIS, testpoints: Vec<TestPoint>) {
    for testpoint in testpoints.iter() {
        let key = testpoint.to_record_key(dut);

        // locate measurement
        match dut.record.measurements.get(&key) {
            Some(measurements) => {
                // browse test points
                for test_measurement in testpoint.measurements.iter() {
                    let mut found = false;

                    match test_measurement {
                        Measurement::ClockOffset(test_offset) => {
                            assert_eq!(measurements.satellite_clock_offset, *test_offset);
                        },
                        Measurement::Observation((test_observable, test_value)) => {
                            // locate
                            match measurements.observations.get(&test_observable) {
                                Some(observed_value) => {
                                    assert_eq!(
                                        observed_value, test_value,
                                        "invalid {} measurement @ {:?}",
                                        test_observable, key
                                    );
                                },
                                None => {
                                    panic!("missing {} observation @ {:?}", test_observable, key);
                                },
                            }
                        },
                    }

                    assert!(
                        found,
                        "missing measurement {:?} @ {:?}",
                        test_measurement, key
                    );
                }
            },
            None => {
                panic!("Failed to locate measurement for {:?}", key);
            },
        }
    }
}
