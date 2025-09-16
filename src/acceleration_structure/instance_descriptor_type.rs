use objc2::{Encode, Encoding, RefEncode};

/// Type of instance descriptor layout used in acceleration structures.
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLAccelerationStructureInstanceDescriptorType {
    /// Default instance descriptor.
    Default = 0,
    /// Instance descriptor with an added user ID.
    UserID = 1,
    /// Instance descriptor with support for motion.
    Motion = 2,
    /// Instance descriptor with a resource handle for the instanced acceleration structure.
    Indirect = 3,
    /// Motion instance descriptor with a resource handle for the instanced acceleration structure.
    IndirectMotion = 4,
}
unsafe impl Encode for MTLAccelerationStructureInstanceDescriptorType {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLAccelerationStructureInstanceDescriptorType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
