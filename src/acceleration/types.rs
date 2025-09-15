use core::ffi::c_float;
use objc2::{Encode, Encoding, RefEncode};

/// Controls the acceleration structure refit operation (from `MTLAccelerationStructureRefitOptions`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLAccelerationStructureRefitOptions(pub u64);

bitflags::bitflags! {
    impl MTLAccelerationStructureRefitOptions: u64 {
        const VertexData = 1<<0;
        const PerPrimitiveData = 1<<1;
    }
}

unsafe impl Encode for MTLAccelerationStructureRefitOptions {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLAccelerationStructureRefitOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Usage flags for an acceleration structure (from `MTLAccelerationStructureUsage`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLAccelerationStructureUsage(pub u64);

bitflags::bitflags! {
    impl MTLAccelerationStructureUsage: u64 {
        const None = 0;
        const Refit = 1<<0;
        const PreferFastBuild = 1<<1;
        const ExtendedLimits = 1<<2;
        const PreferFastIntersection = 1<<4;
        const MinimizeMemory = 1<<5;
    }
}

unsafe impl Encode for MTLAccelerationStructureUsage {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLAccelerationStructureUsage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Per-instance options (from `MTLAccelerationStructureInstanceOptions`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLAccelerationStructureInstanceOptions(pub u32);

bitflags::bitflags! {
    impl MTLAccelerationStructureInstanceOptions: u32 {
        const None = 0;
        const DisableTriangleCulling = 1<<0;
        const TriangleFrontFacingWindingCounterClockwise = 1<<1;
        const Opaque = 1<<2;
        const NonOpaque = 1<<3;
    }
}

unsafe impl Encode for MTLAccelerationStructureInstanceOptions {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for MTLAccelerationStructureInstanceOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Matrix layout (from `MTLMatrixLayout`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLMatrixLayout {
    ColumnMajor = 0,
    RowMajor = 1,
}

unsafe impl Encode for MTLMatrixLayout {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLMatrixLayout {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Motion border mode (from `MTLMotionBorderMode`).
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLMotionBorderMode {
    Clamp = 0,
    Vanish = 1,
}

unsafe impl Encode for MTLMotionBorderMode {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for MTLMotionBorderMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Transform type (from `MTLTransformType`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLTransformType {
    PackedFloat4x3 = 0,
    Component = 1,
}

unsafe impl Encode for MTLTransformType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLTransformType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Curve type (from `MTLCurveType`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCurveType {
    Round = 0,
    Flat = 1,
}

unsafe impl Encode for MTLCurveType {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCurveType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Curve basis (from `MTLCurveBasis`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCurveBasis {
    BSpline = 0,
    CatmullRom = 1,
    Linear = 2,
    Bezier = 3,
}

unsafe impl Encode for MTLCurveBasis {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCurveBasis {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Curve end caps (from `MTLCurveEndCaps`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCurveEndCaps {
    None = 0,
    Disk = 1,
    Sphere = 2,
}

unsafe impl Encode for MTLCurveEndCaps {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCurveEndCaps {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
