//! High-level Rust bindings for Apple's Metal GPU framework.
//!
//! This crate is **work-in-progress** – we currently expose the most fundamental
//! building blocks (`Device`, `CommandQueue`, `CommandBuffer`, `Buffer`).  More
//! APIs arrive as the checklist in `TODO_PORTING_STATUS.md` progresses.

pub use objc2;
pub use objc2_foundation;
pub use block2;

// Public sub-modules
pub mod prelude;
mod device;
mod command_queue;
mod command_buffer;
mod buffer;
pub mod command_buffer_descriptor;

// Public re-exports for ergonomic `use metal_rs::*;`
pub use device::Device;
pub use command_queue::CommandQueue;
pub use command_buffer::{CommandBuffer, CommandBufferStatus, CommandBufferError};
pub use buffer::{Buffer, ResourceOptions as BufferResourceOptions};
pub use command_buffer_descriptor::{CommandBufferDescriptor, CommandBufferErrorOption};