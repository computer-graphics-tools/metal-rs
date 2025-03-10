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

## Project Structure
- Place Foundation types in `src/foundation/`
- Metal types go directly in `src/`
- Examples should be minimal, focused, and well-documented