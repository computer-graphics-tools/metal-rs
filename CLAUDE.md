# metal-rs Development Guide

## Build Commands

- Build library: `cargo build`
- Build with all features: `cargo build --all-features`
- Run tests: `cargo test`
- Run single test: `cargo test <test_name>`
- Generate documentation: `cargo doc --open`
- Run examples: `cargo run --example <example_name>`

## Code Style Guidelines

- Follow the dual-type pattern for Objective-C wrappers (owned type + ref type)
- Use proper memory management with Drop/Clone implementations
- Add comprehensive doc comments with examples for public API
- Use `unsafe` blocks only when necessary, with explanatory comments
- Follow Rust naming conventions: snake_case for functions, CamelCase for types
- Propagate errors with proper Result types rather than panicking
- Prefer strong typing over runtime checks when possible
- Keep import order: standard library, external crates, internal modules
- Avoid excessive ownership transfers; prefer borrowing when appropriate
- Always build the code and fix warnings after adding new code
- When implementing new types, add appropriate `#[allow(dead_code)]` annotations for types that will be used in the future

## Project Structure

- Place Foundation types in `src/foundation/`
- Metal types go in `src/metal/` organized by functionality
- MetalFX types go in `src/metalfx/`
- QuartzCore types go in `src/quartzcore/`
- Examples should be minimal, focused, and well-documented

## Quality Assurance

- Build code with `cargo build` after making changes
- Run tests with `cargo test` when applicable
- Build examples with `cargo build --example <example_name>` after updating related functionality
- Address all warnings or explicitly allow them with appropriate annotations
- When implementing new types, check for name collisions with existing types
- Update UNIMPLEMENTED.md and TODO.md when completing implementation of a type
- Periodically run `find metal-cpp -name "*.hpp" | sort` to list all Metal types in the reference implementation
- Compare C++ headers in metal-cpp/ with implemented Rust types in src/ to track progress

## Tracking Implementation Progress

- Use the following command to list all unimplemented Metal types:

  ```bash
  find metal-cpp/Metal -name "*.hpp" | sort | grep -v "MTLPrivate\|MTLDefines\|MTLHeaderBridge\|Metal.hpp" | sed 's/metal-cpp\/Metal\/\(.*\)\.hpp/\1/'
  ```

- Use the following command to list all implemented Rust types:

  ```bash
  find src/metal -name "*.rs" | sort | grep -v "mod.rs" | sed 's/src\/metal\/\(.*\)\.rs/\1/'
  ```

- Compare these lists to identify what needs to be implemented next
- After implementing a new type, verify it's removed from the UNIMPLEMENTED.md file and marked as completed in TODO.md
