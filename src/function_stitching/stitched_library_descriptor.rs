use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol};

use super::{MTLFunctionStitchingGraph, MTLStitchedLibraryOptions};
use crate::library::MTLFunction;

extern_class!(
    /// Container for the graphs and functions needed to create stitched functions.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStitchedLibraryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLStitchedLibraryDescriptor {}
);

unsafe impl CopyingHelper for MTLStitchedLibraryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLStitchedLibraryDescriptor {}
);

impl MTLStitchedLibraryDescriptor {
    extern_methods!(
        #[unsafe(method(functionGraphs))]
        #[unsafe(method_family = none)]
        pub unsafe fn function_graphs(&self) -> Retained<NSArray<MTLFunctionStitchingGraph>>;

        /// Setter for [`function_graphs`][Self::function_graphs].
        #[unsafe(method(setFunctionGraphs:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_function_graphs(
            &self,
            function_graphs: &NSArray<MTLFunctionStitchingGraph>,
        );

        /// Functions referenced by the graphs.
        #[unsafe(method(functions))]
        #[unsafe(method_family = none)]
        pub unsafe fn functions(&self) -> Retained<NSArray<ProtocolObject<dyn MTLFunction>>>;

        /// Setter for [`functions`][Self::functions].
        #[unsafe(method(setFunctions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_functions(&self, functions: &NSArray<ProtocolObject<dyn MTLFunction>>);

        /// Options for creating the stitched library.
        #[unsafe(method(options))]
        #[unsafe(method_family = none)]
        pub unsafe fn options(&self) -> MTLStitchedLibraryOptions;

        /// Setter for [`options`][Self::options].
        #[unsafe(method(setOptions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_options(&self, options: MTLStitchedLibraryOptions);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLStitchedLibraryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
