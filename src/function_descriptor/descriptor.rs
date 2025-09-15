use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

use super::FunctionOptions;

extern_class!(
    /// Descriptor for locating and specializing a visible function.
    #[unsafe(super(NSObject))]
    #[name = "MTLFunctionDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct FunctionDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for FunctionDescriptor {}
);

unsafe impl CopyingHelper for FunctionDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for FunctionDescriptor {}
);

impl FunctionDescriptor {
    extern_methods!(
        /// Create an autoreleased function descriptor.
        #[unsafe(method(functionDescriptor))]
        #[unsafe(method_family = none)]
        pub fn function_descriptor() -> Retained<FunctionDescriptor>;

        /// The name of the visible function to find.
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub fn name(&self) -> Option<Retained<NSString>>;

        /// Setter for [`name`][Self::name].
        #[unsafe(method(setName:))]
        #[unsafe(method_family = none)]
        pub fn set_name(&self, name: Option<&NSString>);

        /// An optional new name for a visible function to allow reuse with different specializations.
        #[unsafe(method(specializedName))]
        #[unsafe(method_family = none)]
        pub fn specialized_name(&self) -> Option<Retained<NSString>>;

        /// Setter for [`specialized_name`][Self::specialized_name].
        #[unsafe(method(setSpecializedName:))]
        #[unsafe(method_family = none)]
        pub fn set_specialized_name(&self, specialized_name: Option<&NSString>);

        /// Options for creating the Function.
        #[unsafe(method(options))]
        #[unsafe(method_family = none)]
        pub fn options(&self) -> FunctionOptions;

        /// Setter for [`options`][Self::options].
        #[unsafe(method(setOptions:))]
        #[unsafe(method_family = none)]
        pub fn set_options(&self, options: FunctionOptions);
    );
}

/// Methods declared on superclass `NSObject`.
impl FunctionDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
