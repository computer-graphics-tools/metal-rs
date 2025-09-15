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

/// Axis aligned bounding box with min and max points.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLAxisAlignedBoundingBox {
    pub min: MTLPackedFloat3,
    pub max: MTLPackedFloat3,
}

unsafe impl Encode for MTLAxisAlignedBoundingBox {
    const ENCODING: Encoding = Encoding::Struct(
        "_MTLAxisAlignedBoundingBox",
        &[<MTLPackedFloat3>::ENCODING, <MTLPackedFloat3>::ENCODING],
    );
}

unsafe impl RefEncode for MTLAxisAlignedBoundingBox {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

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
