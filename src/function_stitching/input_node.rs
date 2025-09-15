use objc2::{
    extern_class, extern_conformance, extern_methods, extern_protocol,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSUInteger};

extern_protocol!(
    /// Node used in a graph for stitching.
    #[name = "MTLFunctionStitchingNode"]
    pub unsafe trait FunctionStitchingNode: NSObjectProtocol + NSCopying {}
);

extern_class!(
    /// Indexed input node of the produced stitched function.
    #[unsafe(super(NSObject))]
    #[name = "MTLFunctionStitchingInputNode"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct FunctionStitchingInputNode;
);

extern_conformance!(
    unsafe impl FunctionStitchingNode for FunctionStitchingInputNode {}
);

extern_conformance!(
    unsafe impl NSCopying for FunctionStitchingInputNode {}
);

unsafe impl CopyingHelper for FunctionStitchingInputNode {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for FunctionStitchingInputNode {}
);

impl FunctionStitchingInputNode {
    extern_methods!(
        #[unsafe(method(argumentIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn argument_index(&self) -> NSUInteger;

        /// Setter for [`argument_index`][Self::argument_index].
        #[unsafe(method(setArgumentIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_argument_index(&self, argument_index: NSUInteger);

        #[unsafe(method(initWithArgumentIndex:))]
        #[unsafe(method_family = init)]
        pub unsafe fn init_with_argument_index(
            this: Allocated<Self>,
            argument: NSUInteger,
        ) -> Retained<Self>;
    );
}
