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
