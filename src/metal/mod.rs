//! Rust bindings for Apple's Metal framework.
//!
//! This module provides safe Rust wrappers around core Metal types needed for
//! graphics and compute programming on Apple platforms.
//!
//! # Design
//!
//! The implementation follows a dual-type pattern for each Objective-C class:
//! - An owned type (e.g., `MTLDevice`) with ownership semantics
//! - A borrowed reference type (e.g., `MTLDeviceRef`) for borrowed references
//!
//! # Example
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLClearColor};
//! 
//! // Get the default Metal device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a command queue
//! let queue = device.new_command_queue();
//! 
//! // Create a command buffer
//! let command_buffer = queue.new_command_buffer();
//! 
//! // Red clear color
//! let clear_color = MTLClearColor::new(1.0, 0.0, 0.0, 1.0);
//! ```

pub mod device;
pub mod command_queue;
pub mod command_buffer;
pub mod command_encoder;
pub mod types;
pub mod buffer;
pub mod texture;
pub mod library;
pub mod depth_stencil;
pub mod blit_command_encoder;
pub mod compute_command_encoder;

// Re-export types for public API
pub use device::{MTLDevice, MTLDeviceRef, MTLCreateSystemDefaultDevice, MTLFeatureSet};
pub use command_queue::{MTLCommandQueue, MTLCommandQueueRef};
pub use command_buffer::{MTLCommandBuffer, MTLCommandBufferRef, MTLCommandBufferStatus};
pub use command_encoder::{MTLCommandEncoder, MTLCommandEncoderRef};
pub use types::{MTLClearColor, MTLPixelFormat, MTLLoadAction, MTLStoreAction, MTLPrimitiveType};
pub use texture::{MTLSize, MTLOrigin, MTLRegion};
pub use buffer::{MTLBuffer, MTLBufferRef, MTLResourceOptions};
pub use texture::{MTLTexture, MTLTextureRef, MTLTextureType, MTLTextureUsage, MTLTextureDescriptor, MTLTextureDescriptorRef};
pub use library::{MTLLibrary, MTLLibraryRef, MTLFunction, MTLFunctionRef, MTLCompileOptions, MTLFunctionConstantValues, MTLFunctionConstantValuesRef};
pub use depth_stencil::{
    MTLCompareFunction, MTLStencilOperation,
    MTLStencilDescriptor, MTLStencilDescriptorRef,
    MTLDepthStencilDescriptor, MTLDepthStencilDescriptorRef,
    MTLDepthStencilState, MTLDepthStencilStateRef
};
pub use blit_command_encoder::{
    MTLBlitCommandEncoder, MTLBlitCommandEncoderRef, MTLBlitOption
};
pub use compute_command_encoder::{
    MTLComputeCommandEncoder, MTLComputeCommandEncoderRef,
    MTLComputePipelineState, MTLComputePipelineStateRef,
    MTLResource, MTLResourceRef,
    MTLDispatchType, MTLResourceUsage, MTLBarrierScope,
    MTLDispatchThreadgroupsIndirectArguments, MTLStageInRegionIndirectArguments
};