#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [SNR] (Signal to Noise Ratio) for all frequency dependent measurements.
#[derive(Default, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SNR {
    /// SNR ~= 0 dB/Hz
    DbHz0,

    /// SNR < 12 dB/Hz
    DbHz12,

    /// 12 dB/Hz <= SNR < 17 dB/Hz
    DbHz12_17,

    /// 18 dB/Hz <= SNR < 23 dB/Hz
    DbHz18_23,

    /// 24 dB/Hz <= SNR < 29 dB/Hz
    #[default]
    DbHz24_29,

    /// 30 dB/Hz <= SNR < 35 dB/Hz
    DbHz30_35,

    /// 36 dB/Hz <= SNR < 41 dB/Hz
    DbHz36_41,

    /// 42 dB/Hz <= SNR < 47 dB/Hz
    DbHz42_47,

    /// 48 dB/Hz <= SNR < 53 dB/Hz
    DbHz48_53,

    /// SNR >= 54 dB/Hz
    DbHz54,
}
