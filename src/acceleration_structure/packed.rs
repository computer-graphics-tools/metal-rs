use core::ffi::c_float;

use objc2::{Encode, Encoding, RefEncode};

/// Quaternion of 4 f32 values (from `MTLPackedFloatQuaternion`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLPackedFloatQuaternion {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}

unsafe impl Encode for MTLPackedFloatQuaternion {
    const ENCODING: Encoding = Encoding::Struct(
        "MTLPackedFloatQuaternion",
        &[
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLPackedFloatQuaternion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A 3D vector of f32 (from `MTLPackedFloat3`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLPackedFloat3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

unsafe impl Encode for MTLPackedFloat3 {
    const ENCODING: Encoding = Encoding::Struct(
        "_MTLPackedFloat3",
        &[
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLPackedFloat3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A 4x3 matrix of packed float3 columns (from `MTLPackedFloat4x3`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLPackedFloat4x3 {
    pub columns: [MTLPackedFloat3; 4],
}

unsafe impl Encode for MTLPackedFloat4x3 {
    const ENCODING: Encoding =
        Encoding::Struct("_MTLPackedFloat4x3", &[<[MTLPackedFloat3; 4]>::ENCODING]);
}

unsafe impl RefEncode for MTLPackedFloat4x3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
