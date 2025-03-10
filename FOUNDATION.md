# Foundation Library for metal-rs

This document provides an overview of the Foundation library implementation for the metal-rs project.

## Overview

The Foundation library provides Rust bindings to Apple's Foundation framework, focusing on the core types needed for Metal API interaction. The implementation follows patterns from the `metal-rs-master` project and provides safe Rust wrappers around Objective-C objects.

## Core Design Principles

1. **Dual-Type Pattern**: Every Objective-C class is represented by two Rust types:
   - An owned type (e.g., `NSString`) with ownership semantics
   - A borrowed reference type (e.g., `NSStringRef`) for borrowed references

2. **Memory Management**: Proper memory management is ensured through:
   - `Drop` implementations for owned types to release Objective-C objects
   - `Clone` implementations that properly retain objects
   - Use of `foreign-types` traits for handling reference semantics

3. **Type Safety**: The implementation provides type-safe wrappers around Objective-C types and methods:
   - Safe conversion between Rust and Objective-C types
   - Proper error handling
   - Clear ownership semantics

## Implemented Types

Currently, the foundation library includes:

- `NSString`: Safe wrapper around Objective-C's NSString class
- Utility functions for string conversion between Rust and Objective-C

## Usage Example

```rust
use metal_rs::foundation::NSString;

fn main() {
    // Create a string
    let hello = NSString::from_str("Hello, Foundation!");
    
    // Print info about the string
    println!("String length: {}", hello.len());
    println!("Is empty: {}", hello.is_empty());
    println!("Content: {}", hello.as_str());
    
    // String format and conversion
    let formatted = format!("Formatted: {}", hello);
    println!("{}", formatted);
    
    // Clone the string
    let hello_clone = hello.clone();
    println!("Cloned string: {}", hello_clone);
}
```

## Expanding the Library

To add more Foundation types, follow these patterns:

1. Create a new file for each major type in `src/foundation/`
2. Implement the dual-type pattern with `ForeignType` and `ForeignTypeRef`
3. Implement proper memory management through `Drop` and `Clone`
4. Add utility functions for conversion between Rust and Objective-C types
5. Re-export the types in `src/foundation/mod.rs`

## Resources

- [objc crate documentation](https://docs.rs/objc/)
- [foreign-types crate documentation](https://docs.rs/foreign-types/)
- [Apple Foundation Framework Reference](https://developer.apple.com/documentation/foundation)