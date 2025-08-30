use thiserror::Error;

use gnss_rs::{cospar::Error as CosparParsingError, domes::Error as DOMESParsingError};

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

    #[error("failed to parse phase observation flag")]
    ObservationFlag,

    #[error("observable parsing error")]
    Observable,

    #[error("invalid receiver format")]
    Receiver,

    #[error("invalid frequency")]
    Frequency,

    #[error("COSPAR number parsing: {0}")]
    COSPAR(#[from] CosparParsingError),

    #[error("DOMES site number parsing: {0}")]
    DOMES(#[from] DOMESParsingError),

    #[error("L1/L2 date offset parsing error")]
    DorisL1L2DateOffset,

    #[error("ground station parsing error")]
    GroundStation,

    #[error("epoch error: {0}")]
    Epoch(#[from] HifitimeError),

    #[error("invalid epoch format")]
    EpochFormat,

    #[error("epoch parsing error: {0}")]
    EpochParsing(#[from] HifitimeParsingError),

    #[error("not a standardized file name")]
    NonStandardFileName,

    #[error("failed to parse station clock offset")]
    ClockOffset,

    #[error("invalid station format")]
    StationFormat,
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
