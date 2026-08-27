use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{MTLClearColor, MTLRenderPassAttachmentDescriptor};

extern_class!(
    /// Color attachment descriptor for a render pass.
    #[unsafe(super(MTLRenderPassAttachmentDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassColorAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLRenderPassColorAttachmentDescriptor {}
);

unsafe impl CopyingHelper for MTLRenderPassColorAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRenderPassColorAttachmentDescriptor {}
);

impl MTLRenderPassColorAttachmentDescriptor {
    extern_methods!(
        /// The clear color to be used if the load action is Clear.
        #[unsafe(method(clearColor))]
        #[unsafe(method_family = none)]
        pub fn clear_color(&self) -> MTLClearColor;

        /// Setter for [`clear_color`][Self::clear_color].
        #[unsafe(method(setClearColor:))]
        #[unsafe(method_family = none)]
        pub fn set_clear_color(
            &self,
            clear_color: MTLClearColor,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLRenderPassColorAttachmentDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
