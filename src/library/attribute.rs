use objc2::{extern_class, extern_conformance, extern_methods, msg_send, rc::Retained, runtime::NSObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::MTLDataType;

macro_rules! impl_attribute {
    ($attribute:ident) => {
        extern_conformance!(
            unsafe impl NSObjectProtocol for $attribute {}
        );

        impl $attribute {
            extern_methods!(
                /// The index of the attribute.
                #[unsafe(method(attributeIndex))]
                #[unsafe(method_family = none)]
                pub fn attribute_index(&self) -> usize;

                /// The data type of the attribute.
                #[unsafe(method(attributeType))]
                #[unsafe(method_family = none)]
                pub fn attribute_type(&self) -> MTLDataType;

                /// Whether the attribute is active.
                #[unsafe(method(isActive))]
                #[unsafe(method_family = none)]
                pub fn is_active(&self) -> bool;

                /// Whether the attribute contains per-patch data.
                #[unsafe(method(isPatchData))]
                #[unsafe(method_family = none)]
                pub fn is_patch_data(&self) -> bool;

                /// Whether the attribute contains per-patch-control-point data.
                #[unsafe(method(isPatchControlPointData))]
                #[unsafe(method_family = none)]
                pub fn is_patch_control_point_data(&self) -> bool;
            );

            /// The attribute's name.
            pub fn name(&self) -> String {
                let name: Retained<NSString> = unsafe { msg_send![self, name] };
                name.to_string()
            }
        }
    };
}

extern_class!(
    /// Reflection information for a vertex attribute.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVertexAttribute;
);

impl_attribute!(MTLVertexAttribute);

extern_class!(
    /// Reflection information for a stage-input attribute.
    ///
    /// Availability: macOS 10.12+, iOS 10.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAttribute;
);

impl_attribute!(MTLAttribute);
