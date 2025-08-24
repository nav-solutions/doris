use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader, Read},
    str::from_utf8,
};

use crate::prelude::{Comments, Record};

#[cfg(feature = "log")]
use log::error;

impl Record {
    /// Parses the DORIS [Record] content by consuming the [Reader] until the end of stream.
    /// This requires reference to previously parsed [Header] section.
    pub fn parse<R: Read>(
        header: &mut Header,
        reader: &mut BufReader<R>,
    ) -> Result<(Self, Comments), ParsingError> {
        // eos reached: process pending buffer & exit
        let mut eos = false;

        // crinex decompression in failure: process pending buffer & exit
        let mut crinex_error = false;

        // current line storage
        let mut line_buf = String::with_capacity(128);

        // epoch storage
        let mut epoch_buf = String::with_capacity(1024);

        let mut comments = Comments::default();
        let mut record = Record::default();

        let mut gnss_observables = Default::default();

        if let Some(obs) = &header.obs {
            if let Some(crinex) = &obs.crinex {
                is_crinex = true;
                crinex_v3 = crinex.version.major > 2;
            }
            gnss_observables = obs.codes.clone();
        }

        // Iterate and consume, one line at a time
        while let Ok(size) = reader.read_line(&mut line_buf) {
            if size == 0 {
                // reached EOS: consume buffer & exit
                eos |= true;
            }

            if line_buf.len() > 60 {
                if line_buf.contains("COMMENT") {
                    // Comments are stored as is
                    let comment = line_buf.split_at(60).0.trim_end();
                    comments.push(comment.to_string());

                    line_buf.clear();
                    continue; // skip parsing
                }
            }

            // tries to assemble a complete epoch
            let mut new_epoch = false;

            // new epoch being detected or end of stream
            if line_buf.starts_with('>') && epoch_buf.len() > 0 || eos {
                new_epoch = true;
            }

            // clear on new epoch detection
            if new_epoch {
                epoch_buf.clear();
            }

            // always stack new content
            epoch_buf.push_str(&line_buf);

            if eos {
                break;
            }

            line_buf.clear(); // always clear newline buf
        } //while

        Ok(record)
    }
}
