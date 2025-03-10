//! MTLResource - A Rust wrapper around Metal's MTLResource protocol.
//!
//! This module provides safe Rust bindings to the MTLResource protocol from Apple's Metal framework.
//! MTLResource is the abstract base protocol for Metal buffer and texture resources.
//!
//! # Examples
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLResourceOptions, MTLPurgeableState};
//!
//! // Get the default system device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a buffer (which is a MTLResource)
//! let buffer = device.new_buffer(1024, MTLResourceOptions::StorageModeShared);
//! 
//! // Access resource properties
//! println!("Storage mode: {:?}", buffer.storage_mode());
//! println!("CPU cache mode: {:?}", buffer.cpu_cache_mode());
//! 
//! // Set resource label
//! buffer.set_label("My Buffer Resource");
//! 
//! // Control resource memory usage
//! let old_state = buffer.set_purgeable_state(MTLPurgeableState::Volatile);
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSString, NSUInteger};

/// Describes the purgeable state of a resource.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLPurgeableState {
    /// Keep the current state.
    KeepCurrent = 1,
    /// Resource is non-volatile.
    NonVolatile = 2,
    /// Resource is volatile and can be discarded if needed.
    Volatile = 3,
    /// Resource has been purged.
    Empty = 4,
}

/// Describes the CPU cache mode of a resource.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLCPUCacheMode {
    /// The default CPU cache mode.
    DefaultCache = 0,
    /// Write-combined CPU cache mode.
    WriteCombined = 1,
}

/// Describes the storage mode of a resource.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLStorageMode {
    /// Resources are stored in memory accessible by both the CPU and GPU.
    Shared = 0,
    /// Resources are stored in memory accessible by both the CPU and GPU, with automatic migration.
    Managed = 1,
    /// Resources are stored in memory only accessible from the GPU.
    Private = 2,
    /// Resources are transient and only exist during a render pass.
    Memoryless = 3,
}

/// Describes the hazard tracking mode of a resource.
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLHazardTrackingMode {
    /// Use the default hazard tracking mode.
    Default = 0,
    /// No hazard tracking is performed.
    Untracked = 1,
    /// Hazard tracking is performed.
    Tracked = 2,
}

/// Resource options, combining CPU cache mode, storage mode, and hazard tracking.
/// This matches the options used in buffer and texture creation.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLResourceOptions {
    /// Default CPU cache mode, Shared storage mode, Default hazard tracking
    CPUCacheModeDefaultCache = 0,
    /// Write-combined CPU cache mode, Shared storage mode, Default hazard tracking
    CPUCacheModeWriteCombined = 1,
    
    /// Managed storage mode with default CPU cache mode
    StorageModeManaged = 16,
    /// Private storage mode with default CPU cache mode
    StorageModePrivate = 32,
    /// Memoryless storage mode with default CPU cache mode
    StorageModeMemoryless = 48,
    
    /// Hazard tracking is untracked
    HazardTrackingModeUntracked = 256,
    /// Hazard tracking is tracked
    HazardTrackingModeTracked = 512,
    
    /// Shared storage mode with write-combined CPU cache mode (common configuration)
    StorageModeSharedCPUCacheModeWriteCombined = 17,
}

/// A reference to an Objective-C `MTLResource`.
pub struct MTLResourceRef(Object);

/// An owned Objective-C `MTLResource`.
pub struct MTLResource(*mut Object);

unsafe impl ForeignTypeRef for MTLResourceRef {
    type CType = Object;
}

unsafe impl Send for MTLResourceRef {}
unsafe impl Sync for MTLResourceRef {}

unsafe impl ForeignType for MTLResource {
    type CType = Object;
    type Ref = MTLResourceRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLResource {
        MTLResource(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLResourceRef> for MTLResource {
    fn as_ref(&self) -> &MTLResourceRef {
        unsafe { &*(self.0.cast::<MTLResourceRef>()) }
    }
}

unsafe impl Send for MTLResource {}
unsafe impl Sync for MTLResource {}

unsafe impl objc::Message for MTLResourceRef {}

impl Drop for MTLResource {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLResource {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLResource::from_ptr(obj)
        }
    }
}

impl MTLResourceRef {
    /// Returns the label of the resource.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self, label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.to_rust_string())
            }
        }
    }
    
    /// Returns the CPU cache mode of the resource.
    #[must_use]
    pub fn cpu_cache_mode(&self) -> MTLCPUCacheMode {
        unsafe {
            msg_send![self, cpuCacheMode]
        }
    }
    
    /// Returns the storage mode of the resource.
    #[must_use]
    pub fn storage_mode(&self) -> MTLStorageMode {
        unsafe {
            msg_send![self, storageMode]
        }
    }
    
    /// Returns the hazard tracking mode of the resource.
    #[must_use]
    pub fn hazard_tracking_mode(&self) -> MTLHazardTrackingMode {
        unsafe {
            msg_send![self, hazardTrackingMode]
        }
    }
    
    /// Returns the resource options of the resource.
    #[must_use]
    pub fn resource_options(&self) -> MTLResourceOptions {
        unsafe {
            msg_send![self, resourceOptions]
        }
    }
    
    /// Returns the allocated size of the resource in bytes.
    #[must_use]
    pub fn allocated_size(&self) -> NSUInteger {
        unsafe {
            msg_send![self, allocatedSize]
        }
    }
    
    /// Returns whether the resource is aliasable.
    #[must_use]
    pub fn is_aliasable(&self) -> bool {
        unsafe {
            msg_send![self, isAliasable]
        }
    }
}

impl MTLResource {
    /// Returns the label of the resource.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        self.as_ref().label()
    }
    
    /// Sets the label of the resource.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self.as_ref(), setLabel:ns_string.as_ptr()];
        }
    }
    
    /// Returns the CPU cache mode of the resource.
    #[must_use]
    pub fn cpu_cache_mode(&self) -> MTLCPUCacheMode {
        self.as_ref().cpu_cache_mode()
    }
    
    /// Returns the storage mode of the resource.
    #[must_use]
    pub fn storage_mode(&self) -> MTLStorageMode {
        self.as_ref().storage_mode()
    }
    
    /// Returns the hazard tracking mode of the resource.
    #[must_use]
    pub fn hazard_tracking_mode(&self) -> MTLHazardTrackingMode {
        self.as_ref().hazard_tracking_mode()
    }
    
    /// Returns the resource options of the resource.
    #[must_use]
    pub fn resource_options(&self) -> MTLResourceOptions {
        self.as_ref().resource_options()
    }
    
    /// Sets the purgeable state of the resource.
    ///
    /// Returns the previous purgeable state.
    pub fn set_purgeable_state(&self, state: MTLPurgeableState) -> MTLPurgeableState {
        unsafe {
            msg_send![self.as_ref(), setPurgeableState:state]
        }
    }
    
    /// Returns the allocated size of the resource in bytes.
    #[must_use]
    pub fn allocated_size(&self) -> NSUInteger {
        self.as_ref().allocated_size()
    }
    
    /// Makes the resource aliasable.
    pub fn make_aliasable(&self) {
        unsafe {
            let _: () = msg_send![self.as_ref(), makeAliasable];
        }
    }
    
    /// Returns whether the resource is aliasable.
    #[must_use]
    pub fn is_aliasable(&self) -> bool {
        self.as_ref().is_aliasable()
    }
}

impl fmt::Debug for MTLResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.label().unwrap_or_else(|| "Unlabeled".to_string());
        write!(f, "MTLResource {{ label: {}, allocated_size: {} bytes }}", label, self.allocated_size())
    }
}