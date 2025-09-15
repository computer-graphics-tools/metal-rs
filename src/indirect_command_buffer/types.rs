use objc2::{Encode, Encoding, RefEncode};

/// Commands that may be performed indirectly (from `MTLIndirectCommandType`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IndirectCommandType(pub usize);
bitflags::bitflags! {
    impl IndirectCommandType: usize {
        const Draw = 1<<0;
        const DrawIndexed = 1<<1;
        const DrawPatches = 1<<2;
        const DrawIndexedPatches = 1<<3;
        const ConcurrentDispatch = 1<<5;
        const ConcurrentDispatchThreads = 1<<6;
        const DrawMeshThreadgroups = 1<<7;
        const DrawMeshThreads = 1<<8;
    }
}

unsafe impl Encode for IndirectCommandType {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for IndirectCommandType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Execution range for indirect command buffer (from `MTLIndirectCommandBufferExecutionRange`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndirectCommandBufferExecutionRange {
    pub location: u32,
    pub length: u32,
}

unsafe impl Encode for IndirectCommandBufferExecutionRange {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLIndirectCommandBufferExecutionRange=II}",
        &[u32::ENCODING, u32::ENCODING],
    );
}

unsafe impl RefEncode for IndirectCommandBufferExecutionRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
