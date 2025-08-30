use crate::{
    error::FormattingError,
    prelude::{Header, Record},
};

use std::io::{BufWriter, Write};

impl Record {
    pub fn format<W: Write>(
        &self,
        w: &mut BufWriter<W>,
        header: &Header,
    ) -> Result<(), FormattingError> {
        Ok(())
    }
}
