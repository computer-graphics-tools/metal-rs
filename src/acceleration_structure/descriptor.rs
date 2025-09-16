use objc2::{extern_class, runtime::NSObject};

extern_class!(
    /// Minimal wrapper for `MTLAccelerationStructureDescriptor`.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureDescriptor;
);
