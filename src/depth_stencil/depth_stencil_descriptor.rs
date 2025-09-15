use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

use super::{MTLCompareFunction, MTLStencilDescriptor};

extern_class!(
    /// Descriptor for creating a `DepthStencilState`.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLDepthStencilDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLDepthStencilDescriptor {}
);

unsafe impl CopyingHelper for MTLDepthStencilDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLDepthStencilDescriptor {}
);

impl MTLDepthStencilDescriptor {
    extern_methods!(
        #[unsafe(method(depthCompareFunction))]
        #[unsafe(method_family = none)]
        pub fn depth_compare_function(&self) -> MTLCompareFunction;

        #[unsafe(method(setDepthCompareFunction:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_compare_function(&self, value: MTLCompareFunction);

        #[unsafe(method(isDepthWriteEnabled))]
        #[unsafe(method_family = none)]
        pub fn is_depth_write_enabled(&self) -> bool;

        #[unsafe(method(setDepthWriteEnabled:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_write_enabled(&self, value: bool);

        #[unsafe(method(frontFaceStencil))]
        #[unsafe(method_family = none)]
        pub fn front_face_stencil(&self) -> Retained<MTLStencilDescriptor>;

        #[unsafe(method(setFrontFaceStencil:))]
        #[unsafe(method_family = none)]
        pub fn set_front_face_stencil(&self, value: Option<&MTLStencilDescriptor>);

        #[unsafe(method(backFaceStencil))]
        #[unsafe(method_family = none)]
        pub fn back_face_stencil(&self) -> Retained<MTLStencilDescriptor>;

        #[unsafe(method(setBackFaceStencil:))]
        #[unsafe(method_family = none)]
        pub fn set_back_face_stencil(&self, value: Option<&MTLStencilDescriptor>);

        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub fn label(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub fn set_label(&self, label: Option<&NSString>);
    );
}

impl MTLDepthStencilDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
