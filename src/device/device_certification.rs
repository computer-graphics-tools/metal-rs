use objc2::{Encode, Encoding, RefEncode};

/// Device certification tiers.
///
/// Mirrors `NSDeviceCertification` (an `NSInteger`-backed typed enum).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct DeviceCertification(pub i64);

unsafe impl Encode for DeviceCertification {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for DeviceCertification {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[allow(unused)]
unsafe extern "C" {
    static NSDeviceCertificationiPhonePerformanceGaming: i64;
}

/// Returns the certification tier for iPhone Performance Gaming.
#[allow(unused)]
#[inline]
pub fn device_certification_iphone_performance_gaming() -> DeviceCertification {
    unsafe { DeviceCertification(NSDeviceCertificationiPhonePerformanceGaming) }
}
