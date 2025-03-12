//! MTLCounters - A Rust wrapper around Metal's performance counter types.
//!
//! This module provides safe Rust bindings to the counter classes from Apple's Metal framework,
//! allowing performance monitoring and profiling of Metal GPU workloads.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLCounterSamplingPoint};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Check if device supports counter sampling
//! if device.supports_counter_sampling(MTLCounterSamplingPoint::AtDrawBoundary) {
//!     // Get available counter sets
//!     if let Some(counter_sets) = device.counter_sets() {
//!         println!("Number of available counter sets: {}", counter_sets.count());
//!     }
//! }
//! ```

use std::fmt;
use std::os::raw::c_ulonglong;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSArray, NSRange, NSString, NSUInteger};
use crate::metal::resource::MTLStorageMode;
use crate::metal::device::MTLDevice;

/// Represents points where counter sampling can occur.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MTLCounterSamplingPoint {
    /// Sample at stage boundary.
    AtStageBoundary = 0,
    /// Sample at draw boundary.
    AtDrawBoundary = 1,
    /// Sample at dispatch boundary.
    AtDispatchBoundary = 2,
    /// Sample at tile dispatch boundary.
    AtTileDispatchBoundary = 3,
    /// Sample at blit boundary.
    AtBlitBoundary = 4,
}

/// Errors that can occur during counter sample buffer operations.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MTLCounterSampleBufferError {
    /// Out of memory error.
    OutOfMemory = 0,
    /// Invalid descriptor or argument.
    Invalid = 1,
    /// Internal error occurred.
    Internal = 2,
}

/// Timestamp counter result data.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLCounterResultTimestamp {
    /// The timestamp value in GPU cycles.
    pub timestamp: c_ulonglong,
}

/// Stage utilization counter result data.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLCounterResultStageUtilization {
    /// Total cycles across all stages.
    pub total_cycles: c_ulonglong,
    /// Cycles used for vertex processing.
    pub vertex_cycles: c_ulonglong,
    /// Cycles used for tessellation.
    pub tessellation_cycles: c_ulonglong,
    /// Cycles used for post-tessellation vertex processing.
    pub post_tessellation_vertex_cycles: c_ulonglong,
    /// Cycles used for fragment processing.
    pub fragment_cycles: c_ulonglong,
    /// Cycles used for render target operations.
    pub render_target_cycles: c_ulonglong,
}

/// Statistics counter result data.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MTLCounterResultStatistic {
    /// Number of tessellation input patches.
    pub tessellation_input_patches: c_ulonglong,
    /// Number of vertex shader invocations.
    pub vertex_invocations: c_ulonglong,
    /// Number of post-tessellation vertex shader invocations.
    pub post_tessellation_vertex_invocations: c_ulonglong,
    /// Number of clipper invocations.
    pub clipper_invocations: c_ulonglong,
    /// Number of primitives output from the clipper.
    pub clipper_primitives_out: c_ulonglong,
    /// Number of fragment shader invocations.
    pub fragment_invocations: c_ulonglong,
    /// Number of fragments that passed depth/stencil test.
    pub fragments_passed: c_ulonglong,
    /// Number of compute kernel invocations.
    pub compute_kernel_invocations: c_ulonglong,
}

/// Special values for counter sampling.
pub mod counter_sampling {
    use crate::foundation::NSUInteger;
    
    /// Value indicating an error in counter sampling.
    pub const ERROR_VALUE: NSUInteger = NSUInteger::MAX;
    
    /// Value indicating not to sample a counter.
    pub const DONT_SAMPLE: NSUInteger = NSUInteger::MAX - 1;
}

/// A reference to an Objective-C `MTLCounter`.
pub struct MTLCounterRef(Object);

/// An owned Objective-C `MTLCounter`.
pub struct MTLCounter(*mut Object);

unsafe impl ForeignTypeRef for MTLCounterRef {
    type CType = Object;
}

unsafe impl Send for MTLCounterRef {}
unsafe impl Sync for MTLCounterRef {}

unsafe impl ForeignType for MTLCounter {
    type CType = Object;
    type Ref = MTLCounterRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLCounter {
        MTLCounter(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLCounterRef> for MTLCounter {
    fn as_ref(&self) -> &MTLCounterRef {
        unsafe { &*(self.0.cast::<MTLCounterRef>()) }
    }
}

unsafe impl Send for MTLCounter {}
unsafe impl Sync for MTLCounter {}

unsafe impl objc::Message for MTLCounterRef {}

impl MTLCounter {
    /// Gets the name of the counter.
    #[must_use]
    pub fn name(&self) -> String {
        unsafe {
            let name: *mut Object = msg_send![self.as_ref(), name];
            NSString::from_ptr(name).to_string()
        }
    }
}

impl fmt::Debug for MTLCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLCounter")
            .field("name", &self.name())
            .finish()
    }
}

impl Drop for MTLCounter {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLCounter {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLCounter::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLCounterSet`.
pub struct MTLCounterSetRef(Object);

/// An owned Objective-C `MTLCounterSet`.
pub struct MTLCounterSet(*mut Object);

unsafe impl ForeignTypeRef for MTLCounterSetRef {
    type CType = Object;
}

unsafe impl Send for MTLCounterSetRef {}
unsafe impl Sync for MTLCounterSetRef {}

unsafe impl ForeignType for MTLCounterSet {
    type CType = Object;
    type Ref = MTLCounterSetRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLCounterSet {
        MTLCounterSet(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLCounterSetRef> for MTLCounterSet {
    fn as_ref(&self) -> &MTLCounterSetRef {
        unsafe { &*(self.0.cast::<MTLCounterSetRef>()) }
    }
}

unsafe impl Send for MTLCounterSet {}
unsafe impl Sync for MTLCounterSet {}

unsafe impl objc::Message for MTLCounterSetRef {}

impl MTLCounterSet {
    /// Gets the name of the counter set.
    #[must_use]
    pub fn name(&self) -> String {
        unsafe {
            let name: *mut Object = msg_send![self.as_ref(), name];
            NSString::from_ptr(name).to_string()
        }
    }
    
    /// Gets the counters in the counter set.
    #[must_use]
    pub fn counters(&self) -> Option<NSArray> {
        unsafe {
            let counters: *mut Object = msg_send![self.as_ref(), counters];
            if counters.is_null() {
                None
            } else {
                Some(NSArray::from_ptr(counters))
            }
        }
    }
}

impl fmt::Debug for MTLCounterSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLCounterSet")
            .field("name", &self.name())
            .field("counters_count", &self.counters().map(|c| c.count()))
            .finish()
    }
}

impl Drop for MTLCounterSet {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLCounterSet {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLCounterSet::from_ptr(obj)
        }
    }
}

/// A reference to an Objective-C `MTLCounterSampleBufferDescriptor`.
pub struct MTLCounterSampleBufferDescriptorRef(Object);

/// An owned Objective-C `MTLCounterSampleBufferDescriptor`.
pub struct MTLCounterSampleBufferDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLCounterSampleBufferDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLCounterSampleBufferDescriptorRef {}
unsafe impl Sync for MTLCounterSampleBufferDescriptorRef {}

unsafe impl ForeignType for MTLCounterSampleBufferDescriptor {
    type CType = Object;
    type Ref = MTLCounterSampleBufferDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLCounterSampleBufferDescriptor {
        MTLCounterSampleBufferDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLCounterSampleBufferDescriptorRef> for MTLCounterSampleBufferDescriptor {
    fn as_ref(&self) -> &MTLCounterSampleBufferDescriptorRef {
        unsafe { &*(self.0.cast::<MTLCounterSampleBufferDescriptorRef>()) }
    }
}

unsafe impl Send for MTLCounterSampleBufferDescriptor {}
unsafe impl Sync for MTLCounterSampleBufferDescriptor {}

unsafe impl objc::Message for MTLCounterSampleBufferDescriptorRef {}

impl MTLCounterSampleBufferDescriptor {
    /// Creates a new counter sample buffer descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLCounterSampleBufferDescriptor);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            Self::from_ptr(obj)
        }
    }
    
    /// Gets the counter set for the descriptor.
    #[must_use]
    pub fn counter_set(&self) -> Option<MTLCounterSet> {
        unsafe {
            let counter_set: *mut Object = msg_send![self.as_ref(), counterSet];
            if counter_set.is_null() {
                None
            } else {
                Some(MTLCounterSet::from_ptr(counter_set))
            }
        }
    }
    
    /// Sets the counter set for the descriptor.
    pub fn set_counter_set(&self, counter_set: &impl AsRef<MTLCounterSetRef>) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setCounterSet:counter_set.as_ref().as_ptr()];
        }
    }
    
    /// Gets the label of the descriptor.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), label];
            NSString::from_ptr(label).to_string()
        }
    }
    
    /// Sets the label of the descriptor.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
    
    /// Gets the storage mode of the descriptor.
    #[must_use]
    pub fn storage_mode(&self) -> MTLStorageMode {
        unsafe {
            msg_send![self.as_ref(), storageMode]
        }
    }
    
    /// Sets the storage mode of the descriptor.
    pub fn set_storage_mode(&self, storage_mode: MTLStorageMode) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setStorageMode:storage_mode];
        }
    }
    
    /// Gets the sample count of the descriptor.
    #[must_use]
    pub fn sample_count(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), sampleCount]
        }
    }
    
    /// Sets the sample count of the descriptor.
    pub fn set_sample_count(&self, sample_count: NSUInteger) {
        unsafe {
            let _: () = msg_send![self.as_ref(), setSampleCount:sample_count];
        }
    }
}

impl fmt::Debug for MTLCounterSampleBufferDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLCounterSampleBufferDescriptor")
            .field("label", &self.label())
            .field("sample_count", &self.sample_count())
            .field("storage_mode", &self.storage_mode())
            .finish()
    }
}

impl Drop for MTLCounterSampleBufferDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLCounterSampleBufferDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLCounterSampleBufferDescriptor::from_ptr(obj)
        }
    }
}

impl Default for MTLCounterSampleBufferDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

/// A reference to an Objective-C `MTLCounterSampleBuffer`.
pub struct MTLCounterSampleBufferRef(Object);

/// An owned Objective-C `MTLCounterSampleBuffer`.
pub struct MTLCounterSampleBuffer(*mut Object);

unsafe impl ForeignTypeRef for MTLCounterSampleBufferRef {
    type CType = Object;
}

unsafe impl Send for MTLCounterSampleBufferRef {}
unsafe impl Sync for MTLCounterSampleBufferRef {}

unsafe impl ForeignType for MTLCounterSampleBuffer {
    type CType = Object;
    type Ref = MTLCounterSampleBufferRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLCounterSampleBuffer {
        MTLCounterSampleBuffer(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLCounterSampleBufferRef> for MTLCounterSampleBuffer {
    fn as_ref(&self) -> &MTLCounterSampleBufferRef {
        unsafe { &*(self.0.cast::<MTLCounterSampleBufferRef>()) }
    }
}

unsafe impl Send for MTLCounterSampleBuffer {}
unsafe impl Sync for MTLCounterSampleBuffer {}

unsafe impl objc::Message for MTLCounterSampleBufferRef {}

impl MTLCounterSampleBuffer {
    /// Gets the device that created the counter sample buffer.
    #[must_use]
    pub fn device(&self) -> MTLDevice {
        unsafe {
            let device: *mut Object = msg_send![self.as_ref(), device];
            MTLDevice::from_ptr(device)
        }
    }
    
    /// Gets the label of the counter sample buffer.
    #[must_use]
    pub fn label(&self) -> String {
        unsafe {
            let label: *mut Object = msg_send![self.as_ref(), label];
            NSString::from_ptr(label).to_string()
        }
    }
    
    /// Gets the sample count of the counter sample buffer.
    #[must_use]
    pub fn sample_count(&self) -> NSUInteger {
        unsafe {
            msg_send![self.as_ref(), sampleCount]
        }
    }
    
    /// Resolves counter data for a range of samples.
    #[must_use]
    pub fn resolve_counter_range(&self, range: NSRange) -> Option<*mut Object> {
        unsafe {
            let data: *mut Object = msg_send![self.as_ref(), resolveCounterRange:range];
            if data.is_null() {
                None
            } else {
                Some(data)
            }
        }
    }
}

impl fmt::Debug for MTLCounterSampleBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLCounterSampleBuffer")
            .field("label", &self.label())
            .field("sample_count", &self.sample_count())
            .finish()
    }
}

impl Drop for MTLCounterSampleBuffer {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLCounterSampleBuffer {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLCounterSampleBuffer::from_ptr(obj)
        }
    }
}

/// Common counter string constants.
pub mod common_counter {
    /// Timestamp counter.
    pub const TIMESTAMP: &str = "com.apple.metal.counter.timestamp";
    /// Tessellation input patches counter.
    pub const TESSELLATION_INPUT_PATCHES: &str = "com.apple.metal.counter.tessellation.input_patches";
    /// Vertex invocations counter.
    pub const VERTEX_INVOCATIONS: &str = "com.apple.metal.counter.vertex.invocations";
    /// Post-tessellation vertex invocations counter.
    pub const POST_TESSELLATION_VERTEX_INVOCATIONS: &str = "com.apple.metal.counter.post_tessellation.vertex.invocations";
    /// Clipper invocations counter.
    pub const CLIPPER_INVOCATIONS: &str = "com.apple.metal.counter.clipper.invocations";
    /// Clipper primitives out counter.
    pub const CLIPPER_PRIMITIVES_OUT: &str = "com.apple.metal.counter.clipper.primitives_out";
    /// Fragment invocations counter.
    pub const FRAGMENT_INVOCATIONS: &str = "com.apple.metal.counter.fragment.invocations";
    /// Fragments passed counter.
    pub const FRAGMENTS_PASSED: &str = "com.apple.metal.counter.fragment.fragments_passed";
    /// Compute kernel invocations counter.
    pub const COMPUTE_KERNEL_INVOCATIONS: &str = "com.apple.metal.counter.compute.kernel_invocations";
    /// Total cycles counter.
    pub const TOTAL_CYCLES: &str = "com.apple.metal.counter.total.cycles";
    /// Vertex cycles counter.
    pub const VERTEX_CYCLES: &str = "com.apple.metal.counter.vertex.cycles";
    /// Tessellation cycles counter.
    pub const TESSELLATION_CYCLES: &str = "com.apple.metal.counter.tessellation.cycles";
    /// Post-tessellation vertex cycles counter.
    pub const POST_TESSELLATION_VERTEX_CYCLES: &str = "com.apple.metal.counter.post_tessellation.vertex.cycles";
    /// Fragment cycles counter.
    pub const FRAGMENT_CYCLES: &str = "com.apple.metal.counter.fragment.cycles";
    /// Render target write cycles counter.
    pub const RENDER_TARGET_WRITE_CYCLES: &str = "com.apple.metal.counter.render_target.write.cycles";
}

/// Common counter set string constants.
pub mod common_counter_set {
    /// Timestamp counter set.
    pub const TIMESTAMP: &str = "com.apple.metal.counter.set.timestamp";
    /// Stage utilization counter set.
    pub const STAGE_UTILIZATION: &str = "com.apple.metal.counter.set.stage_utilization";
    /// Statistic counter set.
    pub const STATISTIC: &str = "com.apple.metal.counter.set.statistic";
}