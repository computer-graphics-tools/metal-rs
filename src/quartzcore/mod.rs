//! Rust bindings for Apple's QuartzCore framework.
//!
//! This module provides safe Rust wrappers around core QuartzCore types needed for
//! Metal API interaction, particularly CAMetalLayer and CAMetalDrawable.
//!
//! # Design
//!
//! The implementation follows a dual-type pattern for each Objective-C class:
//! - An owned type (e.g., `CAMetalLayer`) with ownership semantics
//! - A borrowed reference type (e.g., `CAMetalLayerRef`) for borrowed references
//!
//! # Example
//!
//! ```no_run
//! use metal_rs::quartzcore::{CAMetalLayer, CGSize, MTLPixelFormat};
//! 
//! // Create a metal layer
//! let layer = CAMetalLayer::new();
//! 
//! // Configure the layer
//! layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
//! layer.set_framebuffer_only(true);
//! 
//! // Set the drawable size
//! let size = CGSize::new(800.0, 600.0);
//! layer.set_drawable_size(size);
//! 
//! // In a real app, you would set a Metal device and get drawables:
//! // layer.set_device(&device);
//! // let drawable = layer.next_drawable();
//! ```

mod metal_drawable;
mod metal_layer;

#[cfg(test)]
mod tests;

// Re-export types for public API
pub use metal_layer::CGSize;
pub use metal_layer::MTLPixelFormat;
pub use metal_drawable::{CAMetalDrawable, CAMetalDrawableRef};
pub use metal_layer::{CAMetalLayer, CAMetalLayerRef};