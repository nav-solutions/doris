use crate::prelude::{ClockOffset, Epoch, EpochFlag, Key, Matcher, Observable, Observation, DORIS};

#[derive(Debug)]
pub enum Measurement {
    ClockOffset(ClockOffset),
    Observation((Observable, Observation)),
}

pub struct TestPoint {
    pub epoch: Epoch,
    pub flag: EpochFlag,
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
            flag: self.flag,
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
                            let sat_offset =
                                measurements.satellite_clock_offset.unwrap_or_else(|| {
                                    panic!("Unreported satellite clock offset @ {:?}", key);
                                });

                            assert_eq!(
                                sat_offset.extrapolated, test_offset.extrapolated,
                                "incorrect clock offset interpretation"
                            );

                            let error = (sat_offset.offset.total_nanoseconds()
                                - test_offset.offset.total_nanoseconds())
                            .abs();

                            assert!(
                                error < 1,
                                "invalid clock offset reported @ {:?} (offset={} err={}ns)",
                                key,
                                sat_offset.offset,
                                error
                            );

                            found = true;
                        },
                        Measurement::Observation((test_observable, test_value)) => {
                            // locate
                            match measurements.observations.get(&test_observable) {
                                Some(observed_value) => {
                                    assert_eq!(
                                        observed_value.snr, test_value.snr,
                                        "invalid {} SNR reported @ {:?}",
                                        test_observable, key
                                    );

                                    let error = (observed_value.value - test_value.value).abs();
                                    assert!(
                                        error < 1.0E-3,
                                        "invalid {} measurement @ {:?} (value={} error={})",
                                        test_observable,
                                        key,
                                        observed_value.value,
                                        error
                                    );

                                    found = true;
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
