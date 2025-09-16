use objc2::{Encode, Encoding, RefEncode};

use super::packed::{MTLPackedFloat3, MTLPackedFloatQuaternion};

/// Transformation represented by scale, shear, pivot, rotation (quaternion) and translation.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLComponentTransform {
    pub scale: MTLPackedFloat3,
    pub shear: MTLPackedFloat3,
    pub pivot: MTLPackedFloat3,
    pub rotation: MTLPackedFloatQuaternion,
    pub translation: MTLPackedFloat3,
}

unsafe impl Encode for MTLComponentTransform {
    const ENCODING: Encoding = Encoding::Struct(
        "MTLComponentTransform",
        &[
            <MTLPackedFloat3>::ENCODING,
            <MTLPackedFloat3>::ENCODING,
            <MTLPackedFloat3>::ENCODING,
            <MTLPackedFloatQuaternion>::ENCODING,
            <MTLPackedFloat3>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLComponentTransform {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
