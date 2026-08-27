use objc2::{extern_class, extern_conformance, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSObject, NSObjectProtocol};

use crate::argument::{MTLArgument, MTLBinding};

extern_class!(
    /// Reflection info for a compute pipeline.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePipelineReflection;
);

// SAFETY: Metal declares `MTLComputePipelineReflection` as `NS_SWIFT_SENDABLE`.
unsafe impl Send for MTLComputePipelineReflection {}
// SAFETY: Metal declares `MTLComputePipelineReflection` as `NS_SWIFT_SENDABLE`.
unsafe impl Sync for MTLComputePipelineReflection {}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLComputePipelineReflection {}
);

impl MTLComputePipelineReflection {
    /// Resource bindings reflected from this compute pipeline.
    pub fn bindings(&self) -> Box<[Retained<ProtocolObject<dyn MTLBinding>>]> {
        let bindings: Retained<NSArray<ProtocolObject<dyn MTLBinding>>> = unsafe { msg_send![self, bindings] };
        bindings.to_vec().into_boxed_slice()
    }

    /// Deprecated argument reflection; use [`bindings`][Self::bindings].
    #[deprecated(note = "use bindings")]
    pub fn arguments(&self) -> Box<[Retained<MTLArgument>]> {
        let arguments: Retained<NSArray<MTLArgument>> = unsafe { msg_send![self, arguments] };
        arguments.to_vec().into_boxed_slice()
    }
}

#[cfg(test)]
mod tests {
    use objc2::{rc::Retained, runtime::ProtocolObject};

    use super::MTLComputePipelineReflection;
    use crate::{MTLArgument, MTLBinding};

    #[test]
    #[expect(deprecated, reason = "verifies the deprecated Rust-native compatibility API")]
    fn collection_methods_have_rust_native_signatures() {
        let _: fn(&MTLComputePipelineReflection) -> Box<[Retained<ProtocolObject<dyn MTLBinding>>]> =
            MTLComputePipelineReflection::bindings;
        let _: fn(&MTLComputePipelineReflection) -> Box<[Retained<MTLArgument>]> =
            MTLComputePipelineReflection::arguments;
    }
}
