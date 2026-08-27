use objc2::{Encode, Encoding, RefEncode};

/// Identify a sample within a pixel. Origin is top-left with a range [0,1) for both x and y.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MTLSamplePosition {
    pub x: f32,
    pub y: f32,
}

impl MTLSamplePosition {
    /// Creates a sample position or two-dimensional coordinate.
    ///
    /// This is the Rust equivalent of Metal's `MTLSamplePositionMake` and
    /// `MTLCoordinate2DMake` inline helpers.
    pub const fn new(
        x: f32,
        y: f32,
    ) -> Self {
        Self {
            x,
            y,
        }
    }
}

unsafe impl Encode for MTLSamplePosition {
    const ENCODING: Encoding = Encoding::Struct("?", &[f32::ENCODING, f32::ENCODING]);
}

unsafe impl RefEncode for MTLSamplePosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
