use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::vertex_buffer_layout_descriptor::VertexBufferLayoutDescriptor;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[name = "MTLVertexBufferLayoutDescriptorArray"]
    pub struct VertexBufferLayoutDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for VertexBufferLayoutDescriptorArray {}
);

impl VertexBufferLayoutDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            index: usize,
        ) -> Retained<VertexBufferLayoutDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            buffer_desc: Option<&VertexBufferLayoutDescriptor>,
            index: usize,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl VertexBufferLayoutDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
