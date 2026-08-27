use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::NSErrorDomain;

unsafe extern "C" {
    /// The error domain for errors that occur while creating a tensor.
    static MTLTensorDomain: &'static NSErrorDomain;
}

/// Returns the error domain for errors that occur while creating a tensor.
#[inline]
pub fn tensor_error_domain() -> String {
    unsafe { MTLTensorDomain }.to_string()
}

/// The error codes that Metal can raise when you create a tensor (from `MTLTensorError`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLTensorError {
    /// No error occurred.
    None = 0,
    /// An internal error occurred.
    InternalError = 1,
    /// The descriptor was invalid.
    InvalidDescriptor = 2,
}

unsafe impl Encode for MTLTensorError {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLTensorError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
