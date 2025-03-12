//! MTLIOCompressor - A Rust wrapper around Metal's I/O compressor API.
//!
//! This module provides Rust bindings to the MTLIOCompressor functionality from Apple's Metal framework.
//! MTLIOCompressor is used for hardware-accelerated compression to enable efficient data transfer.

use std::fmt;
use std::ffi::{CString, c_void};
use std::os::raw::c_char;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::{Object, Class, Protocol};

/// The compression method to use.
#[repr(i64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLIOCompressionMethod {
    /// The zlib compression method.
    Zlib = 0,
    /// The LZFSE compression method.
    LZFSE = 1,
    /// The LZ4 compression method.
    LZ4 = 2,
    /// The LZMA compression method.
    LZMA = 3,
    /// The LZ bitmap compression method.
    LZBitmap = 4,
}

/// The status of a compression operation.
#[repr(i64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MTLIOCompressionStatus {
    /// The operation completed successfully.
    Complete = 0,
    /// The operation failed.
    Error = 1,
}

/// A compression context for compressing data.
pub struct MTLIOCompressionContext {
    context: *mut c_void,
}

impl MTLIOCompressionContext {
    /// Creates a new compression context.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the output file.
    /// * `method` - The compression method to use.
    /// * `chunk_size` - The chunk size to use for compression. If 0, the default chunk size is used.
    ///
    /// # Returns
    ///
    /// A new compression context, or `None` if the context could not be created.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal_rs::metal::{MTLIOCompressionContext, MTLIOCompressionMethod};
    ///
    /// let context = MTLIOCompressionContext::new("/path/to/file.compressed", MTLIOCompressionMethod::LZFSE, 0)
    ///     .expect("Failed to create compression context");
    /// ```
    pub fn new(path: &str, method: MTLIOCompressionMethod, chunk_size: usize) -> Option<Self> {
        let path_cstring = CString::new(path).ok()?;
        let context = unsafe {
            MTLIOCreateCompressionContext(
                path_cstring.as_ptr(),
                method,
                if chunk_size == 0 { MTLIOCompressionContextDefaultChunkSize() } else { chunk_size },
            )
        };
        
        if context.is_null() {
            None
        } else {
            Some(Self { context })
        }
    }
    
    /// Appends data to the compression context.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to append.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal_rs::metal::{MTLIOCompressionContext, MTLIOCompressionMethod};
    ///
    /// let mut context = MTLIOCompressionContext::new("/path/to/file.compressed", MTLIOCompressionMethod::LZFSE, 0)
    ///     .expect("Failed to create compression context");
    ///
    /// let data = [0u8; 1024];
    /// context.append_data(&data);
    /// ```
    pub fn append_data(&self, data: &[u8]) {
        unsafe {
            MTLIOCompressionContextAppendData(
                self.context,
                data.as_ptr() as *const c_void,
                data.len(),
            );
        }
    }
    
    /// Returns the default chunk size for compression contexts.
    ///
    /// # Returns
    ///
    /// The default chunk size in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use metal_rs::metal::MTLIOCompressionContext;
    ///
    /// let default_chunk_size = MTLIOCompressionContext::default_chunk_size();
    /// println!("Default chunk size: {}", default_chunk_size);
    /// ```
    pub fn default_chunk_size() -> usize {
        unsafe { MTLIOCompressionContextDefaultChunkSize() }
    }
    
    /// Flushes and destroys the compression context.
    ///
    /// # Returns
    ///
    /// The status of the operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use metal_rs::metal::{MTLIOCompressionContext, MTLIOCompressionMethod, MTLIOCompressionStatus};
    ///
    /// let mut context = MTLIOCompressionContext::new("/path/to/file.compressed", MTLIOCompressionMethod::LZFSE, 0)
    ///     .expect("Failed to create compression context");
    ///
    /// let data = [0u8; 1024];
    /// context.append_data(&data);
    ///
    /// let status = context.flush_and_destroy();
    /// assert_eq!(status, MTLIOCompressionStatus::Complete);
    /// ```
    pub fn flush_and_destroy(self) -> MTLIOCompressionStatus {
        let status = unsafe { MTLIOFlushAndDestroyCompressionContext(self.context) };
        status
    }
}

impl Drop for MTLIOCompressionContext {
    fn drop(&mut self) {
        if !self.context.is_null() {
            unsafe {
                MTLIOFlushAndDestroyCompressionContext(self.context);
            }
        }
    }
}

impl fmt::Debug for MTLIOCompressionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLIOCompressionContext")
            .field("context", &self.context)
            .finish()
    }
}

// Foreign function declarations
#[link(name = "Metal", kind = "framework")]
unsafe extern "C" {
    /// Returns the default chunk size for compression contexts.
    fn MTLIOCompressionContextDefaultChunkSize() -> usize;
    
    /// Creates a compression context.
    fn MTLIOCreateCompressionContext(
        path: *const c_char,
        method: MTLIOCompressionMethod,
        chunk_size: usize,
    ) -> *mut c_void;
    
    /// Appends data to a compression context.
    fn MTLIOCompressionContextAppendData(
        context: *mut c_void,
        data: *const c_void,
        size: usize,
    );
    
    /// Flushes and destroys a compression context.
    fn MTLIOFlushAndDestroyCompressionContext(
        context: *mut c_void,
    ) -> MTLIOCompressionStatus;
}