use super::FunctionDescriptor;
use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

extern_class!(
    /// Descriptor for an intersection function (subclass of `FunctionDescriptor`).
    #[unsafe(super(FunctionDescriptor, NSObject))]
    #[name = "MTLIntersectionFunctionDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct IntersectionFunctionDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for IntersectionFunctionDescriptor {}
);

unsafe impl CopyingHelper for IntersectionFunctionDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for IntersectionFunctionDescriptor {}
);

impl IntersectionFunctionDescriptor {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl IntersectionFunctionDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
