use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{MTLCompareFunction, MTLStencilOperation};

extern_class!(
    /// Descriptor for stencil state properties.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStencilDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLStencilDescriptor {}
);

unsafe impl CopyingHelper for MTLStencilDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLStencilDescriptor {}
);

impl MTLStencilDescriptor {
    extern_methods!(
        #[unsafe(method(stencilCompareFunction))]
        #[unsafe(method_family = none)]
        pub fn stencil_compare_function(&self) -> MTLCompareFunction;

        #[unsafe(method(setStencilCompareFunction:))]
        #[unsafe(method_family = none)]
        pub fn set_stencil_compare_function(&self, value: MTLCompareFunction);

        #[unsafe(method(stencilFailureOperation))]
        #[unsafe(method_family = none)]
        pub fn stencil_failure_operation(&self) -> MTLStencilOperation;

        #[unsafe(method(setStencilFailureOperation:))]
        #[unsafe(method_family = none)]
        pub fn set_stencil_failure_operation(&self, value: MTLStencilOperation);

        #[unsafe(method(depthFailureOperation))]
        #[unsafe(method_family = none)]
        pub fn depth_failure_operation(&self) -> MTLStencilOperation;

        #[unsafe(method(setDepthFailureOperation:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_failure_operation(&self, value: MTLStencilOperation);

        #[unsafe(method(depthStencilPassOperation))]
        #[unsafe(method_family = none)]
        pub fn depth_stencil_pass_operation(&self) -> MTLStencilOperation;

        #[unsafe(method(setDepthStencilPassOperation:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_stencil_pass_operation(&self, value: MTLStencilOperation);

        #[unsafe(method(readMask))]
        #[unsafe(method_family = none)]
        pub fn read_mask(&self) -> u32;

        #[unsafe(method(setReadMask:))]
        #[unsafe(method_family = none)]
        pub fn set_read_mask(&self, value: u32);

        #[unsafe(method(writeMask))]
        #[unsafe(method_family = none)]
        pub fn write_mask(&self) -> u32;

        #[unsafe(method(setWriteMask:))]
        #[unsafe(method_family = none)]
        pub fn set_write_mask(&self, value: u32);
    );
}

impl MTLStencilDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
