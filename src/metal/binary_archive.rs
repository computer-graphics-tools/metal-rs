// Copyright 2024 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(dead_code)]
use std::fmt::{Debug, Formatter};

use foreign_types::{ForeignType, ForeignTypeRef};
use objc::{
    class, msg_send, runtime::{BOOL, Object, NO, YES}, sel, sel_impl,
};

use crate::foundation::{NSString};
use crate::metal::{
    device::MTLDeviceRef,
    library::MTLLibraryRef,
    function_stitching::MTLStitchedLibraryDescriptor, 
    compute_pipeline::MTLComputePipelineDescriptor,
    render_pipeline::MTLRenderPipelineDescriptor,
    function_descriptor::MTLFunctionDescriptor,
    pipeline::MTLTileRenderPipelineDescriptor,
    pipeline::MTLMeshRenderPipelineDescriptor,
};

/// Binary archive error codes.
#[repr(u64)]
#[derive(Copy, Clone, Debug)]
pub enum MTLBinaryArchiveError {
    /// No error.
    None = 0,
    /// The given binary archive file is invalid or corrupted.
    InvalidFile = 1,
    /// The binary archive contains an unexpected element.
    UnexpectedElement = 2,
    /// The compilation failed when adding functions to the binary archive.
    CompilationFailure = 3,
    /// An internal error occurred in the binary archive implementation.
    InternalError = 4,
}

/// A descriptor that configures a binary archive.
///
/// This descriptor is used to create a new binary archive from a URL.
///
/// # Examples
///
/// ```no_run
/// use metal::{BinaryArchiveDescriptor, Device, NSURL};
///
/// let device = Device::system_default().expect("No device found");
/// let url = NSURL::file_url_with_path("/path/to/archive.metallib", false);
///
/// let descriptor = BinaryArchiveDescriptor::new();
/// descriptor.set_url(&url);
///
/// let binary_archive = device.new_binary_archive_with_descriptor(&descriptor)
///     .expect("Failed to create binary archive");
/// ```
pub struct MTLBinaryArchiveDescriptorRef(Object);

pub struct MTLBinaryArchiveDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLBinaryArchiveDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLBinaryArchiveDescriptorRef {}
unsafe impl Sync for MTLBinaryArchiveDescriptorRef {}

unsafe impl ForeignType for MTLBinaryArchiveDescriptor {
    type CType = Object;
    type Ref = MTLBinaryArchiveDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLBinaryArchiveDescriptor {
        MTLBinaryArchiveDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLBinaryArchiveDescriptorRef> for MTLBinaryArchiveDescriptor {
    fn as_ref(&self) -> &MTLBinaryArchiveDescriptorRef {
        unsafe { &*(self.0.cast::<MTLBinaryArchiveDescriptorRef>()) }
    }
}

unsafe impl Send for MTLBinaryArchiveDescriptor {}
unsafe impl Sync for MTLBinaryArchiveDescriptor {}

unsafe impl objc::Message for MTLBinaryArchiveDescriptorRef {}

impl Drop for MTLBinaryArchiveDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLBinaryArchiveDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = msg_send![self.0, retain];
            MTLBinaryArchiveDescriptor(ptr)
        }
    }
}

impl MTLBinaryArchiveDescriptor {
    /// Creates a new binary archive descriptor.
    ///
    /// # Returns
    ///
    /// A new binary archive descriptor.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLBinaryArchiveDescriptor);
            let alloc: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![alloc, init];
            Self::from_ptr(obj)
        }
    }
}

impl MTLBinaryArchiveDescriptorRef {
    /// Gets the URL of the binary archive.
    ///
    /// # Returns
    ///
    /// The URL where the binary archive is located.
    pub fn url(&self) -> Option<String> {
        unsafe {
            let ptr: *mut Object = msg_send![self, url];
            if ptr.is_null() {
                None
            } else {
                // Get the absolute string from the URL
                let url_string: *mut Object = msg_send![ptr, absoluteString];
                if url_string.is_null() {
                    None
                } else {
                    let ns_string = NSString::from_ptr(url_string);
                    Some(ns_string.as_str().to_string())
                }
            }
        }
    }

    /// Sets the URL of the binary archive.
    ///
    /// # Arguments
    ///
    /// * `url_string` - The URL string where the binary archive is located.
    pub fn set_url(&self, url_string: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(url_string);
            let url_class = class!(NSURL);
            let url: *mut Object = msg_send![url_class, URLWithString: ns_string.as_ptr()];
            if !url.is_null() {
                msg_send![self, setUrl: url]
            }
        }
    }
}

impl Debug for MTLBinaryArchiveDescriptorRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let url = self.url();
        f.debug_struct("MTLBinaryArchiveDescriptor")
            .field("url", &url)
            .finish()
    }
}

/// A container that stores compiled Metal functions for faster loading.
///
/// Binary archives store precompiled Metal functions, making it faster to create and load
/// pipeline states. Use this type to create, load, and save binary archives.
///
/// # Examples
///
/// ```no_run
/// use metal::{BinaryArchiveDescriptor, ComputePipelineDescriptor, Device, NSURL};
///
/// let device = Device::system_default().expect("No device found");
/// let url = NSURL::file_url_with_path("/path/to/archive.metallib", false);
///
/// // Create a binary archive
/// let descriptor = BinaryArchiveDescriptor::new();
/// descriptor.set_url(&url);
///
/// let binary_archive = device.new_binary_archive_with_descriptor(&descriptor)
///     .expect("Failed to create binary archive");
///
/// // Add compute functions to the archive
/// let compute_descriptor = ComputePipelineDescriptor::new();
/// // Configure compute descriptor...
///
/// let _ = binary_archive.add_compute_pipeline_functions(&compute_descriptor);
///
/// // Save the binary archive to disk
/// let save_url = NSURL::file_url_with_path("/path/to/save.metallib", false);
/// let _ = binary_archive.serialize_to_url(&save_url);
/// ```
pub struct MTLBinaryArchiveRef(Object);

pub struct MTLBinaryArchive(*mut Object);

unsafe impl ForeignTypeRef for MTLBinaryArchiveRef {
    type CType = Object;
}

unsafe impl Send for MTLBinaryArchiveRef {}
unsafe impl Sync for MTLBinaryArchiveRef {}

unsafe impl ForeignType for MTLBinaryArchive {
    type CType = Object;
    type Ref = MTLBinaryArchiveRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLBinaryArchive {
        MTLBinaryArchive(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLBinaryArchiveRef> for MTLBinaryArchive {
    fn as_ref(&self) -> &MTLBinaryArchiveRef {
        unsafe { &*(self.0.cast::<MTLBinaryArchiveRef>()) }
    }
}

unsafe impl Send for MTLBinaryArchive {}
unsafe impl Sync for MTLBinaryArchive {}

unsafe impl objc::Message for MTLBinaryArchiveRef {}

impl Drop for MTLBinaryArchive {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLBinaryArchive {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = msg_send![self.0, retain];
            MTLBinaryArchive(ptr)
        }
    }
}

impl MTLBinaryArchiveRef {
    /// Gets the label of the binary archive.
    ///
    /// # Returns
    ///
    /// The label of the binary archive.
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self, label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.as_str().to_string())
            }
        }
    }

    /// Sets the label of the binary archive.
    ///
    /// # Arguments
    ///
    /// * `label` - The label for the binary archive.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            msg_send![self, setLabel: ns_string.as_ptr()]
        }
    }

    /// Gets the device that created this binary archive.
    ///
    /// # Returns
    ///
    /// The device that created this binary archive.
    pub fn device(&self) -> &MTLDeviceRef {
        unsafe {
            let ptr: *mut MTLDeviceRef = msg_send![self, device];
            &*ptr
        }
    }

    /// Adds the functions for a compute pipeline to the binary archive.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor that configures the compute pipeline.
    ///
    /// # Returns
    ///
    /// `true` if the functions were added successfully, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use metal::{BinaryArchive, ComputePipelineDescriptor};
    /// # let binary_archive: BinaryArchive = unimplemented!();
    /// let compute_descriptor = ComputePipelineDescriptor::new();
    /// // Configure compute descriptor...
    ///
    /// let success = binary_archive.add_compute_pipeline_functions(&compute_descriptor);
    /// ```
    pub fn add_compute_pipeline_functions(&self, descriptor: &MTLComputePipelineDescriptor) -> Result<(), String> {
        unsafe {
            let mut error_ptr: *mut Object = std::ptr::null_mut();
            let success: BOOL = msg_send![self, addComputePipelineFunctionsWithDescriptor:descriptor.as_ptr() error:&mut error_ptr];
            
            if success == YES {
                Ok(())
            } else if !error_ptr.is_null() {
                // Extract error description if available
                let description: *mut Object = msg_send![error_ptr, localizedDescription];
                if !description.is_null() {
                    let ns_string = NSString::from_ptr(description);
                    Err(ns_string.as_str().to_string())
                } else {
                    Err("Unknown error".to_string())
                }
            } else {
                Err("Failed to add compute pipeline functions".to_string())
            }
        }
    }

    /// Adds the functions for a render pipeline to the binary archive.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor that configures the render pipeline.
    ///
    /// # Returns
    ///
    /// `true` if the functions were added successfully, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use metal::{BinaryArchive, RenderPipelineDescriptor};
    /// # let binary_archive: BinaryArchive = unimplemented!();
    /// let render_descriptor = RenderPipelineDescriptor::new();
    /// // Configure render descriptor...
    ///
    /// let success = binary_archive.add_render_pipeline_functions(&render_descriptor);
    /// ```
    pub fn add_render_pipeline_functions(&self, descriptor: &MTLRenderPipelineDescriptor) -> Result<(), String> {
        unsafe {
            let mut error_ptr: *mut Object = std::ptr::null_mut();
            let success: BOOL = msg_send![self, addRenderPipelineFunctionsWithDescriptor:descriptor.as_ptr() error:&mut error_ptr];
            
            if success == YES {
                Ok(())
            } else if !error_ptr.is_null() {
                // Extract error description if available
                let description: *mut Object = msg_send![error_ptr, localizedDescription];
                if !description.is_null() {
                    let ns_string = NSString::from_ptr(description);
                    Err(ns_string.as_str().to_string())
                } else {
                    Err("Unknown error".to_string())
                }
            } else {
                Err("Failed to add render pipeline functions".to_string())
            }
        }
    }

    /// Adds the functions for a tile render pipeline to the binary archive.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor that configures the tile render pipeline.
    ///
    /// # Returns
    ///
    /// `true` if the functions were added successfully, `false` otherwise.
    pub fn add_tile_render_pipeline_functions(&self, descriptor: &crate::metal::pipeline::MTLTileRenderPipelineDescriptor) -> Result<(), String> {
        unsafe {
            let mut error_ptr: *mut Object = std::ptr::null_mut();
            let success: BOOL = msg_send![self, addTileRenderPipelineFunctionsWithDescriptor:descriptor.as_ptr() error:&mut error_ptr];
            
            if success == YES {
                Ok(())
            } else if !error_ptr.is_null() {
                // Extract error description if available
                let description: *mut Object = msg_send![error_ptr, localizedDescription];
                if !description.is_null() {
                    let ns_string = NSString::from_ptr(description);
                    Err(ns_string.as_str().to_string())
                } else {
                    Err("Unknown error".to_string())
                }
            } else {
                Err("Failed to add tile render pipeline functions".to_string())
            }
        }
    }

    /// Adds the functions for a mesh render pipeline to the binary archive.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor that configures the mesh render pipeline.
    ///
    /// # Returns
    ///
    /// `true` if the functions were added successfully, `false` otherwise.
    pub fn add_mesh_render_pipeline_functions(&self, descriptor: &crate::metal::pipeline::MTLMeshRenderPipelineDescriptor) -> Result<(), String> {
        unsafe {
            let mut error_ptr: *mut Object = std::ptr::null_mut();
            let success: BOOL = msg_send![self, addMeshRenderPipelineFunctionsWithDescriptor:descriptor.as_ptr() error:&mut error_ptr];
            
            if success == YES {
                Ok(())
            } else if !error_ptr.is_null() {
                // Extract error description if available
                let description: *mut Object = msg_send![error_ptr, localizedDescription];
                if !description.is_null() {
                    let ns_string = NSString::from_ptr(description);
                    Err(ns_string.as_str().to_string())
                } else {
                    Err("Unknown error".to_string())
                }
            } else {
                Err("Failed to add mesh render pipeline functions".to_string())
            }
        }
    }

    /// Adds a stitched library to the binary archive.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor that configures the stitched library.
    ///
    /// # Returns
    ///
    /// `true` if the stitched library was added successfully, `false` otherwise.
    pub fn add_library(&self, descriptor: &MTLStitchedLibraryDescriptor) -> Result<(), String> {
        unsafe {
            let mut error_ptr: *mut Object = std::ptr::null_mut();
            let success: BOOL = msg_send![self, addLibraryWithDescriptor:descriptor.as_ptr() error:&mut error_ptr];
            
            if success == YES {
                Ok(())
            } else if !error_ptr.is_null() {
                // Extract error description if available
                let description: *mut Object = msg_send![error_ptr, localizedDescription];
                if !description.is_null() {
                    let ns_string = NSString::from_ptr(description);
                    Err(ns_string.as_str().to_string())
                } else {
                    Err("Unknown error".to_string())
                }
            } else {
                Err("Failed to add stitched library".to_string())
            }
        }
    }

    /// Adds a function from a library to the binary archive.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor that configures the function.
    /// * `library` - The library containing the function.
    ///
    /// # Returns
    ///
    /// `true` if the function was added successfully, `false` otherwise.
    pub fn add_function(&self, descriptor: &MTLFunctionDescriptor, library: &MTLLibraryRef) -> Result<(), String> {
        unsafe {
            let mut error_ptr: *mut Object = std::ptr::null_mut();
            let success: BOOL = msg_send![self, addFunctionWithDescriptor:descriptor.as_ptr() library:library error:&mut error_ptr];
            
            if success == YES {
                Ok(())
            } else if !error_ptr.is_null() {
                // Extract error description if available
                let description: *mut Object = msg_send![error_ptr, localizedDescription];
                if !description.is_null() {
                    let ns_string = NSString::from_ptr(description);
                    Err(ns_string.as_str().to_string())
                } else {
                    Err("Unknown error".to_string())
                }
            } else {
                Err("Failed to add function".to_string())
            }
        }
    }

    /// Serializes the binary archive to a URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to serialize the binary archive to.
    ///
    /// # Returns
    ///
    /// `true` if the binary archive was serialized successfully, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use metal::{BinaryArchive, NSURL};
    /// # let binary_archive: BinaryArchive = unimplemented!();
    /// let url = NSURL::file_url_with_path("/path/to/save.metallib", false);
    /// let success = binary_archive.serialize_to_url(&url);
    /// ```
    pub fn serialize_to_url(&self, url_string: &str) -> Result<(), String> {
        unsafe {
            let ns_string = NSString::from_rust_str(url_string);
            let url_class = class!(NSURL);
            let url: *mut Object = msg_send![url_class, URLWithString: ns_string.as_ptr()];
            
            if url.is_null() {
                return Err("Invalid URL".to_string());
            }
            
            let mut error_ptr: *mut Object = std::ptr::null_mut();
            let success: BOOL = msg_send![self, serializeToURL:url error:&mut error_ptr];
            
            if success == YES {
                Ok(())
            } else if !error_ptr.is_null() {
                // Extract error description if available
                let description: *mut Object = msg_send![error_ptr, localizedDescription];
                if !description.is_null() {
                    let ns_string = NSString::from_ptr(description);
                    Err(ns_string.as_str().to_string())
                } else {
                    Err("Unknown error".to_string())
                }
            } else {
                Err("Failed to serialize binary archive".to_string())
            }
        }
    }
}

impl Debug for MTLBinaryArchiveRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MTLBinaryArchive")
            .field("label", &self.label())
            .finish()
    }
}