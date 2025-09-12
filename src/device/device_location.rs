use objc2::{Encode, Encoding, RefEncode};

/// Location of the GPU on macOS (ported from `MTLDeviceLocation`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum DeviceLocation {
    BuiltIn = 0,
    Slot = 1,
    External = 2,
    Unspecified = u64::MAX,
}

unsafe impl Encode for DeviceLocation {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for DeviceLocation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}


