use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::Retained,
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{NSError, NSObjectProtocol};

use crate::{
    capture::{CaptureScope, descriptor::CaptureDescriptor, types::CaptureDestination},
    command_queue::CommandQueue,
    device::Device,
};

extern_class!(
    /// Shared capture manager for creating scopes and starting captures.
    #[unsafe(super(NSObject))]
    #[name = "MTLCaptureManager"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CaptureManager;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CaptureManager {}
);

impl CaptureManager {
    extern_methods!(
        /// The shared capture manager for this process.
        #[unsafe(method(sharedCaptureManager))]
        #[unsafe(method_family = none)]
        pub fn shared_capture_manager() -> Retained<CaptureManager>;

        /// Create a capture scope for a device.
        #[unsafe(method(newCaptureScopeWithDevice:))]
        #[unsafe(method_family = new)]
        pub fn new_capture_scope_with_device(
            &self,
            device: &ProtocolObject<dyn Device>,
        ) -> Retained<ProtocolObject<dyn CaptureScope>>;

        /// Create a capture scope for a command queue.
        #[unsafe(method(newCaptureScopeWithCommandQueue:))]
        #[unsafe(method_family = new)]
        pub fn new_capture_scope_with_command_queue(
            &self,
            command_queue: &ProtocolObject<dyn CommandQueue>,
        ) -> Retained<ProtocolObject<dyn CaptureScope>>;

        /// Return whether a destination is supported on this system.
        #[unsafe(method(supportsDestination:))]
        #[unsafe(method_family = none)]
        pub fn supports_destination(&self, destination: CaptureDestination) -> bool;

        /// Start capturing as described by the descriptor.
        #[unsafe(method(startCaptureWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        pub fn start_capture_with_descriptor_error(
            &self,
            descriptor: &CaptureDescriptor,
        ) -> Result<(), Retained<NSError>>;

        /// Stop any ongoing capture.
        #[unsafe(method(stopCapture))]
        #[unsafe(method_family = none)]
        pub fn stop_capture(&self);

        /// Default capture scope.
        #[unsafe(method(defaultCaptureScope))]
        #[unsafe(method_family = none)]
        pub fn default_capture_scope(&self) -> Option<Retained<ProtocolObject<dyn CaptureScope>>>;

        /// Set default capture scope.
        #[unsafe(method(setDefaultCaptureScope:))]
        #[unsafe(method_family = none)]
        pub fn set_default_capture_scope(&self, scope: Option<&ProtocolObject<dyn CaptureScope>>);

        /// Whether a capture is in progress.
        #[unsafe(method(isCapturing))]
        #[unsafe(method_family = none)]
        pub fn is_capturing(&self) -> bool;
    );
}
