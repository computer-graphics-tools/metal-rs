use core::ffi::c_float;
use objc2::{Encode, Encoding, RefEncode};

/// Controls the acceleration structure refit operation (from `MTLAccelerationStructureRefitOptions`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccelerationStructureRefitOptions(pub u64);

bitflags::bitflags! {
    impl AccelerationStructureRefitOptions: u64 {
        const VertexData = 1<<0;
        const PerPrimitiveData = 1<<1;
    }
}

unsafe impl Encode for AccelerationStructureRefitOptions {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for AccelerationStructureRefitOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Usage flags for an acceleration structure (from `MTLAccelerationStructureUsage`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccelerationStructureUsage(pub u64);

bitflags::bitflags! {
    impl AccelerationStructureUsage: u64 {
        const None = 0;
        const Refit = 1<<0;
        const PreferFastBuild = 1<<1;
        const ExtendedLimits = 1<<2;
        const PreferFastIntersection = 1<<4;
        const MinimizeMemory = 1<<5;
    }
}

unsafe impl Encode for AccelerationStructureUsage {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for AccelerationStructureUsage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Per-instance options (from `MTLAccelerationStructureInstanceOptions`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccelerationStructureInstanceOptions(pub u32);

bitflags::bitflags! {
    impl AccelerationStructureInstanceOptions: u32 {
        const None = 0;
        const DisableTriangleCulling = 1<<0;
        const TriangleFrontFacingWindingCounterClockwise = 1<<1;
        const Opaque = 1<<2;
        const NonOpaque = 1<<3;
    }
}

unsafe impl Encode for AccelerationStructureInstanceOptions {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for AccelerationStructureInstanceOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Matrix layout (from `MTLMatrixLayout`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MatrixLayout {
    ColumnMajor = 0,
    RowMajor = 1,
}

unsafe impl Encode for MatrixLayout {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MatrixLayout {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Motion border mode (from `MTLMotionBorderMode`).
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MotionBorderMode {
    Clamp = 0,
    Vanish = 1,
}

unsafe impl Encode for MotionBorderMode {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for MotionBorderMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Transform type (from `MTLTransformType`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransformType {
    PackedFloat4x3 = 0,
    Component = 1,
}

unsafe impl Encode for TransformType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for TransformType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Curve type (from `MTLCurveType`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CurveType {
    Round = 0,
    Flat = 1,
}

unsafe impl Encode for CurveType {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for CurveType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Curve basis (from `MTLCurveBasis`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CurveBasis {
    BSpline = 0,
    CatmullRom = 1,
    Linear = 2,
    Bezier = 3,
}

unsafe impl Encode for CurveBasis {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for CurveBasis {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Curve end caps (from `MTLCurveEndCaps`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CurveEndCaps {
    None = 0,
    Disk = 1,
    Sphere = 2,
}

unsafe impl Encode for CurveEndCaps {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for CurveEndCaps {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
