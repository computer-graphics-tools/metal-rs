use core::ffi::c_void;
use core::ptr::NonNull;

use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSRange, NSString};

use crate::types::DataType;

extern_class!(
    /// Values for Metal function constants (bridged from `MTLFunctionConstantValues`).
    #[unsafe(super(NSObject))]
    #[name = "MTLFunctionConstantValues"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct FunctionConstantValues;
);

extern_conformance!(
    unsafe impl NSCopying for FunctionConstantValues {}
);

unsafe impl CopyingHelper for FunctionConstantValues {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for FunctionConstantValues {}
);

impl FunctionConstantValues {
    extern_methods!(
        /// Set a single constant value by index.
        #[unsafe(method(setConstantValue:type:atIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_constant_value_type_at_index(
            &self,
            value: NonNull<c_void>,
            r#type: DataType,
            index: usize,
        );

        /// Set a range of constant values by index range.
        #[unsafe(method(setConstantValues:type:withRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_constant_values_type_with_range(
            &self,
            values: NonNull<c_void>,
            r#type: DataType,
            range: NSRange,
        );

        /// Set a single constant value by name.
        #[unsafe(method(setConstantValue:type:withName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_constant_value_type_with_name(
            &self,
            value: NonNull<c_void>,
            r#type: DataType,
            name: &NSString,
        );

        /// Reset all function constant values.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);
    );
}

/// Methods declared on superclass `NSObject`.
impl FunctionConstantValues {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
