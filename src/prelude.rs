//! Crate-internal prelude – `use crate::prelude::*;` in every source file.
//!
//! Only types and macros that are ubiquitous across Metal bindings should be
//! re-exported here.  We purposefully keep the list short to avoid name
//! pollution for library users.

#![allow(unused_imports)]

// ObjC runtime & messaging
pub use objc2::{extern_class, extern_protocol, msg_send, sel, ClassType};

// ObjC Foundation layer
pub use objc2_foundation::{NSObject, NSObjectProtocol, NSString};

// Retained pointer helper & autorelease scopes
pub use objc2::rc::{autoreleasepool, Retained};

// Raw Runtime helpers (only when absolutely needed)
pub use objc2::runtime::{AnyObject, ProtocolObject};

// Third-party utilities
pub use bitflags::bitflags;