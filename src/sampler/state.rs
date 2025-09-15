use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{Device, ResourceID};

extern_protocol!(
    /// An immutable collection of sampler state compiled for a single device.
    #[name = "MTLSamplerState"]
    pub unsafe trait SamplerState: NSObjectProtocol + Send + Sync {
        /// A string to help identify this object.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// The device this resource was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn Device>>;

        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpu_resource_id(&self) -> ResourceID;
    }
);
