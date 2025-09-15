use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use super::Architecture;
use crate::{ArgumentBuffersTier, DeviceLocation, ReadWriteTextureTier};

extern_protocol!(
    /// A Metal device that represents a GPU.
    ///
    /// See Apple's documentation for `MTLDevice` for details.
    #[name = "MTLDevice"]
    pub unsafe trait Device: NSObjectProtocol + Send + Sync {
        /// The full name of the vendor device.
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        fn name(&self) -> Retained<NSString>;

        /// The device's architecture information.
        #[unsafe(method(architecture))]
        #[unsafe(method_family = none)]
        unsafe fn architecture(&self) -> Retained<Architecture>;

        /// Whether this GPU is a low-power device.
        #[unsafe(method(isLowPower))]
        #[unsafe(method_family = none)]
        fn is_low_power(&self) -> bool;

        /// Whether this GPU has no displays attached.
        #[unsafe(method(isHeadless))]
        #[unsafe(method_family = none)]
        fn is_headless(&self) -> bool;

        /// Whether this GPU shares its memory with the CPU.
        #[unsafe(method(hasUnifiedMemory))]
        #[unsafe(method_family = none)]
        fn has_unified_memory(&self) -> bool;

        /// Preferred location of the GPU relative to the host.
        #[unsafe(method(location))]
        #[unsafe(method_family = none)]
        fn location(&self) -> DeviceLocation;

        /// Query support tier for read-write texture formats.
        #[unsafe(method(readWriteTextureSupport))]
        #[unsafe(method_family = none)]
        fn read_write_texture_support(&self) -> ReadWriteTextureTier;

        /// Query support tier for argument buffers.
        #[unsafe(method(argumentBuffersSupport))]
        #[unsafe(method_family = none)]
        fn argument_buffers_support(&self) -> ArgumentBuffersTier;
    }
);

/// Returns a reference to the preferred system default Metal device.
#[inline]
pub extern "C-unwind" fn create_system_default_device()
-> Option<Retained<ProtocolObject<dyn Device>>> {
    unsafe extern "C-unwind" {
        fn MTLCreateSystemDefaultDevice() -> *mut ProtocolObject<dyn Device>;
    }
    let ret = unsafe { MTLCreateSystemDefaultDevice() };
    unsafe { Retained::from_raw(ret) }
}
