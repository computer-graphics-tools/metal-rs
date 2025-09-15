use objc2::{Encode, Encoding, RefEncode};

/// Commands that may be performed indirectly (from `MTLIndirectCommandType`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIndirectCommandType(pub usize);
bitflags::bitflags! {
    impl MTLIndirectCommandType: usize {
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

unsafe impl Encode for MTLIndirectCommandType {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLIndirectCommandType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Execution range for indirect command buffer (from `MTLIndirectCommandBufferExecutionRange`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLIndirectCommandBufferExecutionRange {
    pub location: u32,
    pub length: u32,
}

unsafe impl Encode for MTLIndirectCommandBufferExecutionRange {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLIndirectCommandBufferExecutionRange=II}",
        &[u32::ENCODING, u32::ENCODING],
    );
}

unsafe impl RefEncode for MTLIndirectCommandBufferExecutionRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
