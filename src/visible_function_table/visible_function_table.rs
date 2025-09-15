use core::ptr::NonNull;
use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSRange, NSString};

use crate::{MTLResource, MTLResourceID};

extern_protocol!(
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlfunctionhandle?language=objc`
    pub unsafe trait MTLFunctionHandle: NSObjectProtocol + Send + Sync {
        #[unsafe(method(functionType))]
        #[unsafe(method_family = none)]
        fn function_type(&self) -> crate::MTLFunctionType;

        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        fn name(&self) -> Retained<NSString>;

        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn crate::MTLDevice>>;

        /// Handle of the GPU resource suitable for storing in an Intersection Function Buffer.
        /// The handle must have been created from an intersection function annotated with the
        /// `intersection_function_buffer` tag.
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpu_resource_id(&self) -> crate::types::MTLResourceID;
    }
);

extern_protocol!(
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable?language=objc`
    pub unsafe trait MTLVisibleFunctionTable: MTLResource {
        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpu_resource_id(&self) -> MTLResourceID;

        #[unsafe(method(setFunction:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_function_at_index(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunctionHandle>>,
            index: usize,
        );

        /// Safety: `functions` must be a valid pointer.
        #[unsafe(method(setFunctions:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_functions_with_range(
            &self,
            functions: NonNull<*const ProtocolObject<dyn MTLFunctionHandle>>,
            range: NSRange,
        );
    }
);
