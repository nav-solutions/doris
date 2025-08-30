//! Epoch parsing helper

use crate::prelude::{Epoch, ParsingError, TimeScale};

/// Formats given epoch to string, matching standard specifications
pub(crate) fn format(epoch: Epoch, revision: u8) -> String {
    let (y, m, d, hh, mm, ss, nanos) = epoch_decompose(epoch);

    format!(
        "{:04} {:02} {:02} {:02} {:02} {:>2}.{:07}",
        y,
        m,
        d,
        hh,
        mm,
        ss,
        nanos / 100,
    )
}

/// Parses [Epoch] from string, interprated in [TimeScale]
pub(crate) fn parse_in_timescale(content: &str, ts: TimeScale) -> Result<Epoch, ParsingError> {
    let mut y = 0_i32;
    let mut m = 0_u8;
    let mut d = 0_u8;
    let mut hh = 0_u8;
    let mut mm = 0_u8;
    let mut ss = 0_u8;
    let mut ns = 0_u64;

    if content.split_ascii_whitespace().count() < 6 {
        return Err(ParsingError::EpochFormat);
    }

    for (field_index, item) in content.split_ascii_whitespace().enumerate() {
        match field_index {
            0 => {
                y = item.parse::<i32>().map_err(|_| ParsingError::EpochFormat)?;

                /* old RINEX problem: YY sometimes encoded on two digits */
                if y > 79 && y <= 99 {
                    y += 1900;
                } else if y < 79 {
                    y += 2000;
                }
            },
            1 => {
                m = item.parse::<u8>().map_err(|_| ParsingError::EpochFormat)?;
            },
            2 => {
                d = item.parse::<u8>().map_err(|_| ParsingError::EpochFormat)?;
            },
            3 => {
                hh = item.parse::<u8>().map_err(|_| ParsingError::EpochFormat)?;
            },
            4 => {
                mm = item.parse::<u8>().map_err(|_| ParsingError::EpochFormat)?;
            },
            5 => {
                if let Some(dot) = item.find('.') {
                    let is_nav = item.trim().len() < 7;

                    ss = item[..dot]
                        .trim()
                        .parse::<u8>()
                        .map_err(|_| ParsingError::EpochFormat)?;

                    let nanos = item[dot + 1..].trim();

                    ns = nanos
                        .parse::<u64>()
                        .map_err(|_| ParsingError::EpochFormat)?;

                    if is_nav {
                        // NAV RINEX : 100ms precision
                        ns *= 100_000_000;
                    } else if nanos.len() != 9 {
                        // OBS RINEX : 100ns precision
                        ns *= 100;
                    }
                } else {
                    ss = item
                        .trim()
                        .parse::<u8>()
                        .map_err(|_| ParsingError::EpochFormat)?;
                }
            },
            _ => {},
        }
    }

    //println!("content \"{}\"", content); // DEBUG
    //println!("Y {} M {} D {} HH {} MM {} SS {} NS {}", y, m, d, hh, mm, ss, ns); // DEBUG
    match ts {
        TimeScale::UTC => {
            // Catch possible Hifitime panic on bad string content
            if y == 0 {
                return Err(ParsingError::EpochFormat);
            }

            Ok(Epoch::from_gregorian_utc(y, m, d, hh, mm, ss, ns as u32))
        },
        TimeScale::TAI => {
            // Catch possible Hifitime panic on bad string content
            if y == 0 {
                return Err(ParsingError::EpochFormat);
            }
            let epoch = Epoch::from_gregorian_tai(y, m, d, hh, mm, ss, ns as u32);
            Ok(epoch)
        },
        ts => {
            // Catch possible Hifitime panic on bad string content
            if y == 0 {
                return Err(ParsingError::EpochFormat);
            }

            let epoch = Epoch::from_gregorian_str(&format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06} {}",
                y, m, d, hh, mm, ss, ns, ts
            ))?;
            Ok(epoch)
        },
    }
}

pub(crate) fn parse_utc(s: &str) -> Result<Epoch, ParsingError> {
    parse_in_timescale(s, TimeScale::UTC)
}

pub(crate) fn epoch_decompose(epoch: Epoch) -> (i32, u8, u8, u8, u8, u8, u32) {
    epoch.to_gregorian(epoch.time_scale)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::prelude::{Epoch, TimeScale};

    use std::str::FromStr;
}
