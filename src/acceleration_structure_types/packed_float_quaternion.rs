use core::ffi::c_float;

use objc2::{Encode, Encoding, RefEncode};

/// Quaternion of 4 f32 values matching `MTLPackedFloatQuaternion` from Metal.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLPackedFloatQuaternion {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}

impl MTLPackedFloatQuaternion {
    /// Creates a packed quaternion.
    ///
    /// This is the Rust equivalent of Metal's `MTLPackedFloatQuaternionMake`
    /// inline helper.
    pub const fn new(
        x: c_float,
        y: c_float,
        z: c_float,
        w: c_float,
    ) -> Self {
        Self {
            x,
            y,
            z,
            w,
        }
    }
}

unsafe impl Encode for MTLPackedFloatQuaternion {
    const ENCODING: Encoding =
        Encoding::Struct("?", &[<c_float>::ENCODING, <c_float>::ENCODING, <c_float>::ENCODING, <c_float>::ENCODING]);
}

unsafe impl RefEncode for MTLPackedFloatQuaternion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn constructor_and_layout_match_metal() {
        assert_eq!(
            MTLPackedFloatQuaternion::new(1.0, 2.0, 3.0, 4.0),
            MTLPackedFloatQuaternion {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 4.0,
            }
        );
        assert_eq!(size_of::<MTLPackedFloatQuaternion>(), 4 * size_of::<c_float>());
        assert_eq!(align_of::<MTLPackedFloatQuaternion>(), align_of::<c_float>());
    }
}
