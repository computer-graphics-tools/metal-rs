use objc2::{Encode, Encoding, RefEncode};

/// Identify a sample within a pixel. Origin is top-left with a range [0,1) for both x and y.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SamplePosition {
    pub x: f32,
    pub y: f32,
}

unsafe impl Encode for SamplePosition {
    const ENCODING: Encoding = Encoding::Struct("{MTLSamplePosition=ff}", &[f32::ENCODING, f32::ENCODING]);
}

unsafe impl RefEncode for SamplePosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}


