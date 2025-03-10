//! MTLSampler - A Rust wrapper around Metal's MTLSampler class.
//!
//! This module provides safe Rust bindings to the MTLSampler class from Apple's Metal framework.
//! MTLSampler represents a sampler state that defines how a texture is sampled, including filter modes and addressing options.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLSamplerDescriptor, MTLSamplerMinMagFilter, MTLSamplerAddressMode};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a sampler descriptor
//! let descriptor = MTLSamplerDescriptor::new();
//! descriptor.set_min_filter(MTLSamplerMinMagFilter::Linear);
//! descriptor.set_mag_filter(MTLSamplerMinMagFilter::Linear);
//! descriptor.set_s_address_mode(MTLSamplerAddressMode::ClampToEdge);
//! descriptor.set_t_address_mode(MTLSamplerAddressMode::ClampToEdge);
//! 
//! // Create a sampler state using the descriptor
//! let sampler_state = device.new_sampler_state(&descriptor);
//! ```

use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use std::fmt;
use crate::foundation::NSString;
use crate::metal::device::MTLDeviceRef;

/// MTLSamplerMinMagFilter - Enumeration of filter modes for sampler state.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLSamplerMinMagFilter {
    /// Nearest filtering (point sampling)
    Nearest = 0,
    /// Linear filtering
    Linear = 1,
}

/// MTLSamplerMipFilter - Enumeration of mipmap filter modes for sampler state.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLSamplerMipFilter {
    /// No mipmapping
    NotMipmapped = 0,
    /// Nearest mipmap level
    Nearest = 1,
    /// Linear interpolation between mipmap levels
    Linear = 2,
}

/// MTLSamplerAddressMode - Enumeration of address modes for sampler state.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLSamplerAddressMode {
    /// Clamp to edge
    ClampToEdge = 0,
    /// Mirror clamp to edge
    MirrorClampToEdge = 1,
    /// Repeat
    Repeat = 2,
    /// Mirror repeat
    MirrorRepeat = 3,
    /// Clamp to zero
    ClampToZero = 4,
    /// Clamp to border color
    ClampToBorderColor = 5,
}

/// MTLSamplerBorderColor - Enumeration of border colors for sampler state.
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLSamplerBorderColor {
    /// Transparent black
    TransparentBlack = 0,
    /// Opaque black
    OpaqueBlack = 1,
    /// Opaque white
    OpaqueWhite = 2,
}

/// A reference to an Objective-C `MTLSamplerDescriptor`.
pub struct MTLSamplerDescriptorRef(Object);

/// An owned Objective-C `MTLSamplerDescriptor`.
pub struct MTLSamplerDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLSamplerDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLSamplerDescriptorRef {}
unsafe impl Sync for MTLSamplerDescriptorRef {}

unsafe impl ForeignType for MTLSamplerDescriptor {
    type CType = Object;
    type Ref = MTLSamplerDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLSamplerDescriptor {
        MTLSamplerDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLSamplerDescriptorRef> for MTLSamplerDescriptor {
    fn as_ref(&self) -> &MTLSamplerDescriptorRef {
        unsafe { &*(self.0.cast::<MTLSamplerDescriptorRef>()) }
    }
}

unsafe impl Send for MTLSamplerDescriptor {}
unsafe impl Sync for MTLSamplerDescriptor {}

unsafe impl objc::Message for MTLSamplerDescriptorRef {}

impl MTLSamplerDescriptor {
    /// Creates a new sampler descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLSamplerDescriptor);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            MTLSamplerDescriptor::from_ptr(obj)
        }
    }

    /// Sets the minification filter for the sampler state.
    pub fn set_min_filter(&self, filter: MTLSamplerMinMagFilter) {
        unsafe {
            msg_send![self.as_ref(), setMinFilter:filter]
        }
    }

    /// Gets the minification filter for the sampler state.
    #[must_use]
    pub fn min_filter(&self) -> MTLSamplerMinMagFilter {
        unsafe {
            msg_send![self.as_ref(), minFilter]
        }
    }

    /// Sets the magnification filter for the sampler state.
    pub fn set_mag_filter(&self, filter: MTLSamplerMinMagFilter) {
        unsafe {
            msg_send![self.as_ref(), setMagFilter:filter]
        }
    }

    /// Gets the magnification filter for the sampler state.
    #[must_use]
    pub fn mag_filter(&self) -> MTLSamplerMinMagFilter {
        unsafe {
            msg_send![self.as_ref(), magFilter]
        }
    }

    /// Sets the mipmap filter for the sampler state.
    pub fn set_mip_filter(&self, filter: MTLSamplerMipFilter) {
        unsafe {
            msg_send![self.as_ref(), setMipFilter:filter]
        }
    }

    /// Gets the mipmap filter for the sampler state.
    #[must_use]
    pub fn mip_filter(&self) -> MTLSamplerMipFilter {
        unsafe {
            msg_send![self.as_ref(), mipFilter]
        }
    }

    /// Sets the S address mode for the sampler state.
    pub fn set_s_address_mode(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send![self.as_ref(), setSAddressMode:mode]
        }
    }

    /// Gets the S address mode for the sampler state.
    #[must_use]
    pub fn s_address_mode(&self) -> MTLSamplerAddressMode {
        unsafe {
            msg_send![self.as_ref(), sAddressMode]
        }
    }

    /// Sets the T address mode for the sampler state.
    pub fn set_t_address_mode(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send![self.as_ref(), setTAddressMode:mode]
        }
    }

    /// Gets the T address mode for the sampler state.
    #[must_use]
    pub fn t_address_mode(&self) -> MTLSamplerAddressMode {
        unsafe {
            msg_send![self.as_ref(), tAddressMode]
        }
    }

    /// Sets the R address mode for the sampler state.
    pub fn set_r_address_mode(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send![self.as_ref(), setRAddressMode:mode]
        }
    }

    /// Gets the R address mode for the sampler state.
    #[must_use]
    pub fn r_address_mode(&self) -> MTLSamplerAddressMode {
        unsafe {
            msg_send![self.as_ref(), rAddressMode]
        }
    }

    /// Sets the border color for the sampler state.
    pub fn set_border_color(&self, color: MTLSamplerBorderColor) {
        unsafe {
            msg_send![self.as_ref(), setBorderColor:color]
        }
    }

    /// Gets the border color for the sampler state.
    #[must_use]
    pub fn border_color(&self) -> MTLSamplerBorderColor {
        unsafe {
            msg_send![self.as_ref(), borderColor]
        }
    }

    /// Sets whether to use normalized coordinates.
    pub fn set_normalized_coordinates(&self, normalized: bool) {
        unsafe {
            msg_send![self.as_ref(), setNormalizedCoordinates:normalized]
        }
    }

    /// Returns whether to use normalized coordinates.
    #[must_use]
    pub fn normalized_coordinates(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), normalizedCoordinates]
        }
    }

    /// Sets the minimum level of detail (LOD) value.
    pub fn set_lod_min_clamp(&self, lod_min_clamp: f32) {
        unsafe {
            msg_send![self.as_ref(), setLodMinClamp:lod_min_clamp]
        }
    }

    /// Gets the minimum level of detail (LOD) value.
    #[must_use]
    pub fn lod_min_clamp(&self) -> f32 {
        unsafe {
            msg_send![self.as_ref(), lodMinClamp]
        }
    }

    /// Sets the maximum level of detail (LOD) value.
    pub fn set_lod_max_clamp(&self, lod_max_clamp: f32) {
        unsafe {
            msg_send![self.as_ref(), setLodMaxClamp:lod_max_clamp]
        }
    }

    /// Gets the maximum level of detail (LOD) value.
    #[must_use]
    pub fn lod_max_clamp(&self) -> f32 {
        unsafe {
            msg_send![self.as_ref(), lodMaxClamp]
        }
    }

    /// Sets the maximum anisotropy value.
    pub fn set_max_anisotropy(&self, max_anisotropy: u64) {
        unsafe {
            msg_send![self.as_ref(), setMaxAnisotropy:max_anisotropy]
        }
    }

    /// Gets the maximum anisotropy value.
    #[must_use]
    pub fn max_anisotropy(&self) -> u64 {
        unsafe {
            msg_send![self.as_ref(), maxAnisotropy]
        }
    }

    /// Sets whether to use LOD averaging.
    pub fn set_lod_average(&self, lod_average: bool) {
        unsafe {
            msg_send![self.as_ref(), setLodAverage:lod_average]
        }
    }

    /// Returns whether to use LOD averaging.
    #[must_use]
    pub fn lod_average(&self) -> bool {
        unsafe {
            msg_send![self.as_ref(), lodAverage]
        }
    }

    /// Sets the compare function.
    pub fn set_compare_function(&self, compare_function: crate::metal::depth_stencil::MTLCompareFunction) {
        unsafe {
            msg_send![self.as_ref(), setCompareFunction:compare_function]
        }
    }

    /// Gets the compare function.
    #[must_use]
    pub fn compare_function(&self) -> crate::metal::depth_stencil::MTLCompareFunction {
        unsafe {
            msg_send![self.as_ref(), compareFunction]
        }
    }

    /// Sets whether the sampler supports argument buffers.
    pub fn set_support_argument_buffers(&self, support: bool) {
        unsafe {
            msg_send![self.as_ref(), setSupportArgumentBuffers:support]
        }
    }

    /// Returns whether the sampler supports argument buffers.
    #[must_use]
    pub fn support_argument_buffers(&self) -> bool {
        unsafe {
            let result: bool = msg_send![self.as_ref(), supportArgumentBuffers];
            result
        }
    }

    /// Sets the label for the sampler descriptor.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            msg_send![self.as_ref(), setLabel:ns_string.as_ptr()]
        }
    }

    /// Gets the label for the sampler descriptor.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let ns_string: *mut Object = msg_send![self.as_ref(), label];
            if ns_string.is_null() {
                return String::new();
            }
            let ns_string = NSString::from_ptr(ns_string);
            ns_string.to_rust_string()
        }
    }
}

impl Drop for MTLSamplerDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLSamplerDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLSamplerDescriptor::from_ptr(obj)
        }
    }
}

impl fmt::Debug for MTLSamplerDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLSamplerDescriptor")
            .field("min_filter", &self.min_filter())
            .field("mag_filter", &self.mag_filter())
            .field("mip_filter", &self.mip_filter())
            .field("s_address_mode", &self.s_address_mode())
            .field("t_address_mode", &self.t_address_mode())
            .field("r_address_mode", &self.r_address_mode())
            .field("normalized_coordinates", &self.normalized_coordinates())
            .field("lod_min_clamp", &self.lod_min_clamp())
            .field("lod_max_clamp", &self.lod_max_clamp())
            .field("max_anisotropy", &self.max_anisotropy())
            .finish()
    }
}

/// A reference to an Objective-C `MTLSamplerState`.
pub struct MTLSamplerStateRef(Object);

/// An owned Objective-C `MTLSamplerState`.
pub struct MTLSamplerState(*mut Object);

unsafe impl ForeignTypeRef for MTLSamplerStateRef {
    type CType = Object;
}

unsafe impl Send for MTLSamplerStateRef {}
unsafe impl Sync for MTLSamplerStateRef {}

unsafe impl ForeignType for MTLSamplerState {
    type CType = Object;
    type Ref = MTLSamplerStateRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLSamplerState {
        MTLSamplerState(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLSamplerStateRef> for MTLSamplerState {
    fn as_ref(&self) -> &MTLSamplerStateRef {
        unsafe { &*(self.0.cast::<MTLSamplerStateRef>()) }
    }
}

unsafe impl Send for MTLSamplerState {}
unsafe impl Sync for MTLSamplerState {}

unsafe impl objc::Message for MTLSamplerStateRef {}

impl MTLSamplerState {
    /// Returns the device that created this sampler state.
    #[must_use]
    pub fn device(&self) -> &MTLDeviceRef {
        unsafe {
            let device: *mut Object = msg_send![self.as_ref(), device];
            &*(device as *mut MTLDeviceRef)
        }
    }

    /// Returns the label for this sampler state.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let ns_string: *mut Object = msg_send![self.as_ref(), label];
            if ns_string.is_null() {
                return String::new();
            }
            let ns_string = NSString::from_ptr(ns_string);
            ns_string.to_rust_string()
        }
    }
}

impl fmt::Debug for MTLSamplerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MTLSamplerState {{ label: \"{}\" }}", self.label())
    }
}

impl Drop for MTLSamplerState {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLSamplerState {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLSamplerState::from_ptr(obj)
        }
    }
}