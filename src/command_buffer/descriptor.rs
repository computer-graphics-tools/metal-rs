use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

extern_class!(
    /// Properties for creating a `CommandBuffer`.
    #[unsafe(super(NSObject))]
    #[name = "MTLCommandBufferDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CommandBufferDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for CommandBufferDescriptor {}
);

unsafe impl CopyingHelper for CommandBufferDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CommandBufferDescriptor {}
);

impl CommandBufferDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
