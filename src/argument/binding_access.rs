use objc2::{Encode, Encoding, RefEncode};

/// Read/write permissions for resource bindings (from `MTLBindingAccess`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLBindingAccess {
    ReadOnly = 0,
    ReadWrite = 1,
    WriteOnly = 2,
}

unsafe impl Encode for MTLBindingAccess {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLBindingAccess {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
