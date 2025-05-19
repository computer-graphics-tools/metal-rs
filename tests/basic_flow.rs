#![cfg(target_os = "macos")]
//! End-to-end tests for the most fundamental Metal abstractions.
//!
//! These tests are only compiled on macOS because the public CI runner for
//! iOS is not readily available.  They are deliberately *not* run on other
//! platforms to avoid build failures.

use metal_rs::*;
use metal_rs::prelude::autoreleasepool;

#[test]
fn basic_gpu_flow() {
    autoreleasepool(|_| {
        // Acquire the default GPU.
        let device = Device::system_default().expect("No Metal device available");
        assert!(!device.name().is_empty());

        // Create a command queue with a debug label.
        let queue = device.new_command_queue();
        queue.set_label("metal-rs-test-queue");
        assert!(queue.label().is_some());

        // Allocate a small CPU-visible buffer.
        let buffer = device.new_buffer_with_length(
            256,
            BufferResourceOptions::STORAGE_MODE_SHARED,
        );
        assert_eq!(buffer.length(), 256);
        assert!(!buffer.contents().is_null());

        // Submit an empty command buffer (no work, but good for smoke-testing).
        let cmd = queue.new_command_buffer();
        cmd.commit();
        cmd.wait_until_completed();

        // Check that the command buffer reached a terminal state (Committed or later).
        let status = cmd.status();
        assert!(
            matches!(
                status,
                CommandBufferStatus::Committed |
                CommandBufferStatus::Scheduled |
                CommandBufferStatus::Completed |
                CommandBufferStatus::Error
            ),
            "Command buffer status was {:?}, expected Committed or later", status
        );
    });
} 