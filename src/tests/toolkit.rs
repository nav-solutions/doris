use crate::prelude::{ClockOffset, Epoch, EpochFlag, Key, Matcher, Observable, Observation, DORIS};

#[derive(Debug)]
pub struct StationObservationData {
    pub station: u16,
    pub observable: Observable,
    pub value: f64,
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
            TestData::StationObservation(station_data) => {},
        }
    }
}
// // locate
// match measurements.observations.get(&test_observable) {
//     Some(observed_value) => {
//         assert_eq!(
//             observed_value.snr, test_value.snr,
//             "invalid {} SNR reported @ {:?}",
//                     test_observable, key
//                 );

//                 let error = (observed_value.value - test_value.value).abs();
//                 assert!(
//                     error < 1.0E-3,
//                     "invalid {} measurement @ {:?} (value={} error={})",
//                     test_observable,
//                     key,
//                     observed_value.value,
//                     error
//                 );

//                 found = true;
//             },
//             None => {
//                 panic!("missing {} observation @ {:?}", test_observable, key);
//             },
//         }
//     },
// }

// assert!(
//     found,
//     "missing measurement {:?} @ {:?}",
//     test_measurement, key
// );
