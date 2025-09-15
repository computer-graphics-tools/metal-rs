use core::ffi::c_float;
use objc2::{Encode, Encoding, RefEncode};

/// Quaternion of 4 f32 values (from `MTLPackedFloatQuaternion`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PackedFloatQuaternion {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}

unsafe impl Encode for PackedFloatQuaternion {
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

unsafe impl RefEncode for PackedFloatQuaternion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A 3D vector of f32 (from `MTLPackedFloat3`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PackedFloat3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

unsafe impl Encode for PackedFloat3 {
    const ENCODING: Encoding = Encoding::Struct(
        "_MTLPackedFloat3",
        &[
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for PackedFloat3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A 4x3 matrix of packed float3 columns (from `MTLPackedFloat4x3`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PackedFloat4x3 {
    pub columns: [PackedFloat3; 4],
}

unsafe impl Encode for PackedFloat4x3 {
    const ENCODING: Encoding =
        Encoding::Struct("_MTLPackedFloat4x3", &[<[PackedFloat3; 4]>::ENCODING]);
}

unsafe impl RefEncode for PackedFloat4x3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Axis aligned bounding box with min and max points.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AxisAlignedBoundingBox {
    pub min: PackedFloat3,
    pub max: PackedFloat3,
}

unsafe impl Encode for AxisAlignedBoundingBox {
    const ENCODING: Encoding = Encoding::Struct(
        "_MTLAxisAlignedBoundingBox",
        &[<PackedFloat3>::ENCODING, <PackedFloat3>::ENCODING],
    );
}

unsafe impl RefEncode for AxisAlignedBoundingBox {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Transformation represented by scale, shear, pivot, rotation (quaternion) and translation.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComponentTransform {
    pub scale: PackedFloat3,
    pub shear: PackedFloat3,
    pub pivot: PackedFloat3,
    pub rotation: PackedFloatQuaternion,
    pub translation: PackedFloat3,
}

unsafe impl Encode for ComponentTransform {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <PackedFloat3>::ENCODING,
            <PackedFloat3>::ENCODING,
            <PackedFloat3>::ENCODING,
            <PackedFloatQuaternion>::ENCODING,
            <PackedFloat3>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for ComponentTransform {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
