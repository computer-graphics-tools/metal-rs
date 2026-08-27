use core::ffi::c_float;

use objc2::{Encode, Encoding, RefEncode};

/// Packed 3D float vector matching `MTLPackedFloat3` from Metal.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLPackedFloat3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

impl MTLPackedFloat3 {
    /// Creates a packed three-component vector.
    ///
    /// This is the Rust equivalent of Metal's `MTLPackedFloat3Make` inline
    /// helper.
    pub const fn new(
        x: c_float,
        y: c_float,
        z: c_float,
    ) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

unsafe impl Encode for MTLPackedFloat3 {
    const ENCODING: Encoding = Encoding::Struct("?", &[<c_float>::ENCODING, <c_float>::ENCODING, <c_float>::ENCODING]);
}

unsafe impl RefEncode for MTLPackedFloat3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn constructor_and_layout_match_metal() {
        assert_eq!(
            MTLPackedFloat3::new(1.0, 2.0, 3.0),
            MTLPackedFloat3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
        assert_eq!(size_of::<MTLPackedFloat3>(), 3 * size_of::<c_float>());
        assert_eq!(align_of::<MTLPackedFloat3>(), align_of::<c_float>());
    }
}
