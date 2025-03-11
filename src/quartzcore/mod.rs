//\! Rust bindings for Apple's QuartzCore (Core Animation) framework.
//\!
//\! This module provides safe Rust wrappers around core QuartzCore types needed for
//\! graphics programming on Apple platforms.

pub mod metal_layer;

// Re-export types for public API
pub use metal_layer::{CAMetalLayer, CAMetalLayerRef, CAMetalDrawable, CAMetalDrawableRef};
