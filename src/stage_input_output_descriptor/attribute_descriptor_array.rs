use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::AttributeDescriptor;

extern_class!(
    /// Array of attribute descriptors
    #[unsafe(super(NSObject))]
    #[name = "MTLAttributeDescriptorArray"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AttributeDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for AttributeDescriptorArray {}
);

impl AttributeDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            index: usize,
        ) -> Retained<AttributeDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            attribute_desc: Option<&AttributeDescriptor>,
            index: usize,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl AttributeDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
