use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

extern_class!(
    /// Specifies the parameters for `ResidencySet` creation.
    ///
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor?language=objc`
    #[unsafe(super(NSObject))]
    #[name = "MTLResidencySetDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ResidencySetDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for ResidencySetDescriptor {}
);

unsafe impl CopyingHelper for ResidencySetDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for ResidencySetDescriptor {}
);

impl ResidencySetDescriptor {
    extern_methods!(
        /// Optional label for the residency set.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label]. Copied when set.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub fn set_label(&self, label: Option<&NSString>);

        /// If non-zero, defines the number of allocations for which to initialize the internal arrays. Defaults to zero.
        #[unsafe(method(initialCapacity))]
        #[unsafe(method_family = none)]
        pub fn initial_capacity(&self) -> usize;

        /// Setter for [`initial_capacity`][Self::initial_capacity].
        #[unsafe(method(setInitialCapacity:))]
        #[unsafe(method_family = none)]
        pub fn set_initial_capacity(&self, initial_capacity: usize);
    );
}

/// Methods declared on superclass `NSObject`.
impl ResidencySetDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
