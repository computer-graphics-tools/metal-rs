use objc2::{extern_class, extern_conformance, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

extern_class!(
    /// Reflection info for a compute pipeline.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePipelineReflection;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLComputePipelineReflection {}
);
