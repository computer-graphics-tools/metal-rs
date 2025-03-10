//! Rust bindings for Apple's Foundation framework.
//!
//! This module provides safe Rust wrappers around core Foundation types needed for
//! Metal API interaction. It implements proper memory management and conversion
//! between Rust and Objective-C types.
//!
//! # Design
//!
//! The implementation follows a dual-type pattern for each Objective-C class:
//! - An owned type (e.g., `NSString`) with ownership semantics
//! - A borrowed reference type (e.g., `NSStringRef`) for borrowed references
//!
//! # Example
//!
//! ```
//! use metal_rs::foundation::NSString;
//!
//! // Create a string
//! let hello = NSString::from_rust_str("Hello, Foundation!");
//! 
//! // Get string properties
//! let length = hello.len();
//! let content = hello.as_str();
//! 
//! // Use in standard Rust code
//! println!("String: {} (length: {})", content, length);
//! ```

mod string;
mod types;

// Re-export
pub use string::{NSString, NSStringRef, nsstring_from_str, nsstring_as_str};
pub use types::{NSRange, NSPoint, NSSize, NSRect, NSInteger, NSUInteger};