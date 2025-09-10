//! High-level Rust bindings for Apple's Metal GPU framework.
//!
//! This crate is **work-in-progress** – we currently expose the most fundamental
//! building blocks (`Device`, `CommandQueue`, `CommandBuffer`, `Buffer`).  More
//! APIs arrive as the checklist in `TODO_PORTING_STATUS.md` progresses.

pub use objc2;
pub use objc2_foundation;
pub use block2;
