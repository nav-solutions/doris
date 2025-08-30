use thiserror::Error;

use gnss_rs::{
    constellation::ParsingError as ConstellationParsingError, cospar::Error as CosparParsingError,
    domes::Error as DOMESParsingError, sv::ParsingError as SVParsingError,
};

use hifitime::{HifitimeError, ParsingError as HifitimeParsingError};

use std::io::Error as IoError;

/// Errors that may rise when parsing DORIS files
#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("invalid doris file")]
    InvalidDoris,

    #[error("header line too short (invalid)")]
    HeaderLineTooShort,

    #[error("file version parsing error")]
    Version,

    #[error("observable parsing error")]
    Observable,

    #[error("COSPAR number parsing: {0}")]
    COSPAR(#[from] CosparParsingError),

    #[error("DOMES site number parsing: {0}")]
    DOMES(#[from] DOMESParsingError),

    #[error("L1/L2 date offset parsing error")]
    DorisL1L2DateOffset,

    #[error("station parsing error")]
    DorisStation,

    #[error("epoch error: {0}")]
    Epoch(#[from] HifitimeError),

    #[error("invalid epoch format")]
    EpochFormat,

    #[error("epoch parsing error: {0}")]
    EpochParsing(#[from] HifitimeParsingError),
}

/// Errors that may rise when formatting DORIS files
#[derive(Error, Debug)]
pub enum FormattingError {
    #[error("i/o: output error")]
    OutputError(#[from] IoError),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to determine sampling rate")]
    UndeterminedSamplingRate,
}
