use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

extern_class!(
    /// Intersection function table descriptor
    #[unsafe(super(NSObject))]
    #[name = "MTLIntersectionFunctionTableDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct IntersectionFunctionTableDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for IntersectionFunctionTableDescriptor {}
);

unsafe impl CopyingHelper for IntersectionFunctionTableDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for IntersectionFunctionTableDescriptor {}
);

impl IntersectionFunctionTableDescriptor {
    extern_methods!(
        /// Create a descriptor
        #[unsafe(method(intersectionFunctionTableDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn intersection_function_table_descriptor()
        -> Retained<IntersectionFunctionTableDescriptor>;

        /// The number of functions in the table.
        #[unsafe(method(functionCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn function_count(&self) -> usize;

        /// Setter for [`function_count`][Self::function_count].
        #[unsafe(method(setFunctionCount:))]
        #[unsafe(method_family = none)]
        pub fn set_function_count(&self, function_count: usize);
    );
}
