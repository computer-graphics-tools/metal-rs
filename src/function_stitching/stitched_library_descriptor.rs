use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol};

use super::{FunctionStitchingGraph, StitchedLibraryOptions};
use crate::library::Function;

extern_class!(
    /// Container for the graphs and functions needed to create stitched functions.
    #[unsafe(super(NSObject))]
    #[name = "MTLStitchedLibraryDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct StitchedLibraryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for StitchedLibraryDescriptor {}
);

unsafe impl CopyingHelper for StitchedLibraryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for StitchedLibraryDescriptor {}
);

impl StitchedLibraryDescriptor {
    extern_methods!(
        #[unsafe(method(functionGraphs))]
        #[unsafe(method_family = none)]
        pub unsafe fn function_graphs(&self) -> Retained<NSArray<FunctionStitchingGraph>>;

        /// Setter for [`function_graphs`][Self::function_graphs].
        #[unsafe(method(setFunctionGraphs:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_function_graphs(&self, function_graphs: &NSArray<FunctionStitchingGraph>);

        /// Functions referenced by the graphs.
        #[unsafe(method(functions))]
        #[unsafe(method_family = none)]
        pub unsafe fn functions(&self) -> Retained<NSArray<ProtocolObject<dyn Function>>>;

        /// Setter for [`functions`][Self::functions].
        #[unsafe(method(setFunctions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_functions(&self, functions: &NSArray<ProtocolObject<dyn Function>>);

        /// Options for creating the stitched library.
        #[unsafe(method(options))]
        #[unsafe(method_family = none)]
        pub unsafe fn options(&self) -> StitchedLibraryOptions;

        /// Setter for [`options`][Self::options].
        #[unsafe(method(setOptions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_options(&self, options: StitchedLibraryOptions);
    );
}

/// Methods declared on superclass `NSObject`.
impl StitchedLibraryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
