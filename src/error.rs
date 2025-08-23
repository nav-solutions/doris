use thiserror::Error;

use gnss_rs::{
    constellation::ParsingError as ConstellationParsingError, cospar::Error as CosparParsingError,
    domes::Error as DOMESParsingError, sv::ParsingError as SVParsingError,
};

use hifitime::{HifitimeError, ParsingError as HifitimeParsingError};

use std::io::Error as IoError;

use crate::hatanaka::Error as HatanakaError;

/// Errors that may rise when parsing DORIS files
#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("header line too short (invalid)")]
    HeaderLineTooShort,

    #[error("empty epoch")]
    EmptyEpoch,

    #[error("invalid epoch flag")]
    EpochFlag,

    #[error("number of sat")]
    NumSat,

    #[error("epoch parsing")]
    EpochParsing,

    #[error("datime parsing")]
    DatetimeParsing,

    #[error("file version parsing error")]
    VersionParsing,

    #[error("observable parsing")]
    ObservableParsing,

    #[error("DOMES site number parsing: {0}")]
    DOMES(#[from] DOMESParsingError),

    #[error("L1/L2 date offset parsing error")]
    DorisL1L2DateOffset,

    #[error("station parsing error")]
    DorisStation,
}

/// Errors that may rise when formatting DORIS files
#[derive(Error, Debug)]
pub enum FormattingError {
    #[error("i/o: output error")]
    OutputError(#[from] IoError),
}
