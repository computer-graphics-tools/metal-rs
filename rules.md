## Rust module and file layout rules

- **File names without ObjC prefixes**: Use snake_case files and module names. Example: `MTLDevice.h` → `src/device/device.rs` (type `Device`), `MTLRenderPipeline.h` → `src/render_pipeline/render_pipeline.rs` (type `RenderPipeline`).
- **One type per file**: Split large headers into multiple files; each file declares exactly one public type plus its impl/traits.
- **Per-type modules**: For complex APIs create a directory module. Example for `MTLDevice`:
  - Directory: `src/device/`
  - Files: `device.rs`, `gpu_family.rs`, `device_location.rs`, `…`
  - Re-exports in `src/device/mod.rs` using `pub use self::{device::Device, gpu_family::GpuFamily, device_location::DeviceLocation};`
- **Module re-exports**: Each directory module exposes a clean public surface via its `mod.rs` with `pub use` so consumers can `use crate::device::Device` without reaching into submodules.
- **Idiomatic Rust naming**: Public types in `PascalCase`, modules/files in `snake_case`. Keep ObjC names only where necessary for selectors and linking; the Rust API should be idiomatic and prefix-free.
- **Doc comments from headers**: Mirror Objective-C `@brief` and important notes as Rust doc comments on types, fields, and enum variants; link to Apple docs where helpful.

## ObjC → Rust porting TODO

This document tracks the process of porting Metal Objective-C headers in `external/Headers` into idiomatic Rust APIs backed by `objc2` and related crates.

## Workflow

- **Inventory header**: Identify classes, protocols, enums, bitflags, constants, typedefs, and availability.
- **Design Rust API**: Map Objective-C types to Rust types; plan module placement under `src/`.
- **Low-level bindings**: Expose raw ObjC interfaces with `objc2` and safe wrappers around them.
- **Enums/bitflags**: Translate enums and options to Rust enums and `bitflags` as appropriate.
- **Methods/properties**: Implement getters/setters and methods with correct ownership and safety.
- **Availability/feature gates**: Gate APIs by OS version or feature flags.
- **Safety audit**: Document unsafe surfaces; prefer safe wrappers when possible.
- **Tests/examples**: Add unit tests and minimal usage examples.
- **Docs**: Add rustdoc linking to Apple docs; note differences from ObjC.