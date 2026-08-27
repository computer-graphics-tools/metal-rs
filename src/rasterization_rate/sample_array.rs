use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{NSNumber, NSObjectProtocol};

extern_class!(
    /// Helper object for convenient access to samples stored in an array.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRasterizationRateSampleArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRasterizationRateSampleArray {}
);

impl MTLRasterizationRateSampleArray {
    /// Retrieves the single-precision sample value at `index`.
    ///
    /// Metal returns `0.0` when `index` is out of range.
    pub fn object_at_indexed_subscript(
        &self,
        index: usize,
    ) -> f32 {
        let value: Retained<NSNumber> = unsafe { msg_send![self, objectAtIndexedSubscript: index] };
        value.as_f32()
    }

    /// Stores a sample value at `index`.
    pub fn set_object_at_indexed_subscript(
        &self,
        value: f32,
        index: usize,
    ) {
        let value = NSNumber::new_f32(value);
        unsafe {
            let _: () = msg_send![self, setObject: &*value, atIndexedSubscript: index];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTLRasterizationRateSampleArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
