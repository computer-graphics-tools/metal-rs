use objc2::{Encode, Encoding, RefEncode};

/// The level of the log entry (from `MTLLogLevel`).
#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogLevel {
    Undefined = 0,
    Debug = 1,
    Info = 2,
    Notice = 3,
    Error = 4,
    Fault = 5,
}

unsafe impl Encode for LogLevel {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for LogLevel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
