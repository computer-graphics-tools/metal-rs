use objc2::{Encode, Encoding, RefEncode};

/// Describes how a resource will be used by a shader through an argument buffer (from `MTLResourceUsage`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceUsage(pub u64);
bitflags::bitflags! {
    impl ResourceUsage: u64 {
        const Read = 1<<0;
        const Write = 1<<1;
        const Sample = 1<<2; // deprecated upstream but keep for compatibility
    }
}

unsafe impl Encode for ResourceUsage {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for ResourceUsage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes the types of resources that a barrier operates on (from `MTLBarrierScope`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BarrierScope(pub u64);
bitflags::bitflags! {
    impl BarrierScope: u64 {
        const Buffers = 1<<0;
        const Textures = 1<<1;
        const RenderTargets = 1<<2;
    }
}

unsafe impl Encode for BarrierScope {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for BarrierScope {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes stages of GPU work (from `MTLStages`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Stages(pub u64);
bitflags::bitflags! {
    impl Stages: u64 {
        const Vertex = 1<<0;
        const Fragment = 1<<1;
        const Tile = 1<<2;
        const Object = 1<<3;
        const Mesh = 1<<4;
        const ResourceState = 1<<26;
        const Dispatch = 1<<27;
        const Blit = 1<<28;
        const AccelerationStructure = 1<<29;
        const MachineLearning = 1<<30;
        const All = u64::MAX; // matches NSIntegerMax bitmask intent
    }
}

unsafe impl Encode for Stages {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for Stages {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
