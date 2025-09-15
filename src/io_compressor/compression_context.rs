use core::ffi::{c_char, c_void};
use core::ptr::NonNull;

use crate::device::IoCompressionMethod;
use crate::io_compressor::CompressionStatus;

/// Opaque handle to a Metal I/O compression context.
#[repr(transparent)]
pub struct CompressionContext(*mut c_void);

unsafe extern "C-unwind" {
    fn MTLIOCompressionContextDefaultChunkSize() -> usize;
}

unsafe extern "C-unwind" {
    /// Safety: `path` must be a valid, null-terminated C string.
    fn MTLIOCreateCompressionContext(
        path: NonNull<c_char>,
        r#type: IoCompressionMethod,
        chunk_size: usize,
    ) -> *mut c_void;
}

unsafe extern "C-unwind" {
    /// Safety: `context` and `data` must be valid pointers.
    fn MTLIOCompressionContextAppendData(
        context: *mut c_void,
        data: NonNull<c_void>,
        size: usize,
    );
}

unsafe extern "C-unwind" {
    /// Safety: `context` must be a valid pointer.
    fn MTLIOFlushAndDestroyCompressionContext(context: *mut c_void) -> CompressionStatus;
}

impl CompressionContext {
    /// Returns Apple's default chunk size for the compression context.
    pub fn default_chunk_size() -> usize {
        unsafe { MTLIOCompressionContextDefaultChunkSize() }
    }

    /// Create a new compression context that writes to the file at `path`.
    ///
    /// Safety: `path` must be a valid, null-terminated C string pointer.
    pub unsafe fn create(
        path: NonNull<c_char>,
        method: IoCompressionMethod,
        chunk_size: usize,
    ) -> Option<Self> {
        let raw = unsafe { MTLIOCreateCompressionContext(path, method, chunk_size) };
        if raw.is_null() {
            None
        } else {
            Some(Self(raw))
        }
    }

    /// Append raw data to the compression stream.
    ///
    /// Safety: `data` must be valid for reads of `size` bytes for the duration of the call.
    pub unsafe fn append_data(&mut self, data: NonNull<c_void>, size: usize) {
        unsafe { MTLIOCompressionContextAppendData(self.0, data, size) };
    }

    /// Flush pending data and destroy the context, returning the final status.
    /// The handle becomes invalid after this call.
    pub unsafe fn flush_and_destroy(self) -> CompressionStatus {
        let status = unsafe { MTLIOFlushAndDestroyCompressionContext(self.0) };
        core::mem::forget(self);
        status
    }
}

impl Drop for CompressionContext {
    fn drop(&mut self) {
        // Best-effort: avoid double-drop by consuming in `flush_and_destroy`.
        // If user forgot to call it, attempt to flush and destroy here.
        unsafe {
            let _ = MTLIOFlushAndDestroyCompressionContext(self.0);
        }
    }
}


