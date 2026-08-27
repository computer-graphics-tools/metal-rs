use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::NSErrorDomain;

/// Error codes that Metal device methods can produce (from `MTLDeviceError`).
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLDeviceError {
    /// No error occurred.
    None = 0,
    /// The device does not support the requested feature.
    NotSupported = 1,
}

unsafe impl Encode for MTLDeviceError {
    const ENCODING: Encoding = isize::ENCODING;
}

unsafe impl RefEncode for MTLDeviceError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe extern "C" {
    /// The error domain for errors produced by Metal device methods.
    static MTLDeviceErrorDomain: &'static NSErrorDomain;
}

/// Returns the error domain for errors produced by Metal device methods.
#[inline]
pub fn device_error_domain() -> String {
    unsafe { MTLDeviceErrorDomain }.to_string()
}

#[cfg(test)]
mod tests {
    use super::MTLDeviceError;

    #[test]
    fn error_codes_match_metal_header_values() {
        assert_eq!(MTLDeviceError::None as isize, 0);
        assert_eq!(MTLDeviceError::NotSupported as isize, 1);
    }
}
