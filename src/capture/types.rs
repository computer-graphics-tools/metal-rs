use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::NSErrorDomain;

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCaptureError {
    NotSupported = 1,
    AlreadyCapturing = 2,
    InvalidDescriptor = 3,
}

unsafe impl Encode for MTLCaptureError {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLCaptureError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCaptureDestination {
    DeveloperTools = 1,
    GPUTraceDocument = 2,
}

unsafe impl Encode for MTLCaptureDestination {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLCaptureDestination {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe extern "C" {
    static MTLCaptureErrorDomain: &'static NSErrorDomain;
}

#[inline]
pub fn capture_error_domain() -> &'static NSErrorDomain {
    unsafe { MTLCaptureErrorDomain }
}
