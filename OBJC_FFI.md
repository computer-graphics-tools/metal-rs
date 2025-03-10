# Objective-C FFI in Rust

This document summarizes the approach used for Objective-C FFI in the metal-rs project, based on patterns from the metal-rs-master project.

## Core Architecture

### Key Dependencies

- **objc** crate: Provides low-level access to the Objective-C runtime and message passing
- **foreign-types** crate: Manages ownership and reference semantics
- **core-foundation** crate: Provides bindings to Core Foundation types

### The Dual-Type Pattern

Every Objective-C class is represented by two Rust types:

1. **Owned type** (e.g., `NSString`): Represents an owned reference with Drop semantics
2. **Borrowed type** (e.g., `NSStringRef`): Represents a borrowed reference

This is implemented through the `foreign_obj_type!` macro:

```rust
foreign_obj_type! {
    type CType = objc::runtime::Object;
    pub struct NSString;
    pub struct NSStringRef;
    type ParentType = NSObjectRef;  // Optional inheritance
}
```

### Message Sending

The `msg_send!` macro from the `objc` crate is used to send messages to Objective-C objects:

```rust
unsafe {
    let result: ReturnType = msg_send![object, selector:withArgument:, arg1, arg2];
}
```

For BOOL return types, a custom `msg_send_bool!` macro converts Objective-C BOOL to Rust bool:

```rust
pub fn is_equal(&self, other: &NSObjectRef) -> bool {
    msg_send_bool!(self.as_ref(), isEqual:, other)
}
```

### Memory Management

- **Ownership**: ForeignType trait implementations handle retain/release
- **Autorelease**: NSAutoreleasePool is wrapped for RAII-style management
- **Temporary objects**: Creation methods automatically handle ownership

### Error Handling

A `try_objc!` macro is used for methods that may return errors:

```rust
pub fn some_operation() -> Result<ReturnType, NSError> {
    try_objc!([self.as_ref(), operationWithError:], error)
}
```

## Common Patterns

### String Handling

- `nsstring_from_str`: Converts Rust strings to NSString
- `nsstring_as_str`: Converts NSString to borrowed Rust strings
- Convenience methods for string manipulation

### Collections

- Arrays and dictionaries use generic parameter types
- Collections provide iterators that yield borrowed references
- Type-safe conversion between Rust and Objective-C collections

### Type Conversion

- `AsRef` and `From` traits for convenient conversions
- Specialized traits like `ToNSString` for domain-specific conversions
- Careful handling of lifetimes for borrowed references

## Safety Considerations

1. **Nullability**: Check for null pointers when returning optional types
2. **Runtime type checking**: Prefer safe casts when possible
3. **Lifetime management**: Ensure borrowed references don't outlive their owners
4. **Threading**: Consider autorelease pools in threaded contexts

## Implementation Strategy

1. Start with core types (NSObject, NSString)
2. Implement collections and utility types
3. Add specialized domain-specific types
4. Create higher-level safe abstractions

## Example Usage

```rust
// Create an autorelease pool for temporary objects
let pool = AutoreleasePool::new();

// String handling
let hello = NSString::from_str("Hello, World!");
println!("String: {}", hello.as_str());

// Collection management
let array = NSArray::from_slice(&[&str1, &str2, &str3]);
for item in array.into_iter() {
    // Use item
}

// Drop pool when done
drop(pool);
```
