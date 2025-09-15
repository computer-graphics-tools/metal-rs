## Objective-C to Rust API refactor progress

### Goals
- Remove generated `MTL*`-prefixed bindings, but leave `MTL4` in favor of idiomatic Rust modules and names.
- Drop all feature gating (`#[cfg(feature = …)]`) across the public API we port.
- Map ObjC integer/pointer types to idiomatic Rust types (e.g., `NSUInteger → usize`).
- Keep raw `objc2` surfaces in low-level wrappers with safe, idiomatic re-exports.
- Locate MTL4 related types and modules inside ml4 mod

### New/ported modules
- visible_function_table/
  - `VisibleFunctionTableDescriptor`, `VisibleFunctionTable`, `FunctionHandle`
  - Removed cfgs; snake_case methods; re-exported in `lib.rs`.

- heap/
  - `HeapType`, `HeapDescriptor` (existing), new `Heap` protocol trait.
  - Added minimal acceleration structures bridge types.

- fence.rs
  - `Fence` protocol, wired and re-exported.

- types/
  - `DataType` already existed; removed `src/MTLDataType.rs`.

- pipeline/
  - `Mutability`, `ShaderValidation` enums.
  - `PipelineBufferDescriptor`, `PipelineBufferDescriptorArray`.

- resource_view_pool/
  - `ResourceViewPoolDescriptor`, `ResourceViewPool`, `TextureViewPool`.

- stage_input_output_descriptor/
  - `AttributeFormat`, `StepFunction`, `BufferLayoutDescriptor(+Array)`, `AttributeDescriptor(+Array)`, `StageInputOutputDescriptor`.

- sampler/
  - Enums: `MinMagFilter`, `MipFilter`, `AddressMode`, `BorderColor`, `ReductionMode`.
  - `SamplerDescriptor`, `SamplerState`.

- resource_state_pass/
  - `ResourceStatePassSampleBufferAttachmentDescriptor(+Array)`, `ResourceStatePassDescriptor`.

- resource_state_command_encoder/
  - `SparseTextureMappingMode`, `MapIndirectArguments`, `ResourceStateCommandEncoder`.

- compute_command_encoder/
  - Indirect structs: `DispatchThreadgroupsIndirectArguments`, `DispatchThreadsIndirectArguments`, `StageInRegionIndirectArguments`.
  - `DispatchType` bridge enum.
  - `ComputeCommandEncoder` trait (uses `CommandEncoder` base trait).

- command_encoder/
  - `CommandEncoder` base trait.
  - Flags: `ResourceUsage`, `BarrierScope`, `Stages`.

- intersection_function_table/
  - `IntersectionFunctionBufferArguments`, `IntersectionFunctionSignature` flags.
  - `IntersectionFunctionTableDescriptor`, `IntersectionFunctionTable` trait.
  - Uses `visible_function_table::FunctionHandle` re-export.

- rasterization_rate/
  - `RasterizationRateSampleArray`, `RasterizationRateLayerDescriptor`, `RasterizationRateLayerArray`.
  - `RasterizationRateMapDescriptor`, `RasterizationRateMap`.

- tile_render_pipeline/
  - `TileRenderPipelineColorAttachmentDescriptor(+Array)`, `TileRenderPipelineDescriptor`.
  - Mirrors render pipeline patterns; snake_case methods; re-exported in `lib.rs`.

### Device updates
- device/
  - Expanded `Device` trait with core, cfg-free methods.
  - Added `Architecture` class.
  - Added `create_system_default_device()` helper.

### Bridge protocols (minimal)
- `compute_pipeline.rs`: `ComputePipelineState`.
- `indirect_command_buffer.rs`: `IndirectCommandBuffer`.
- `counters.rs`: `CounterSampleBuffer`.

### Deletions (ported and removed)
- `src/MTLVisibleFunctionTable.rs`
- `src/MTLHeap.rs`
- `src/MTLFence.rs`
- `src/MTLBuffer.rs`
- `src/MTLDataType.rs`
- `src/MTLAllocation.rs`
- `src/MTLPixelFormat.rs` (already had `pixel_format.rs`)
- `src/MTLDevice.rs`
- `src/MTLPipeline.rs`
- `src/MTLTextureViewPool.rs`, `src/MTLResourceViewPool.rs`
- `src/MTLStageInputOutputDescriptor.rs`
- `src/MTLSampler.rs`
- `src/MTLResourceStatePass.rs`
- `src/MTLResourceStateCommandEncoder.rs`
- `src/MTLComputeCommandEncoder.rs`
- `src/MTLRasterizationRate.rs`

### Conventions adopted
- Public Rust types in `PascalCase`; modules/files in `snake_case`.
- Method names in `snake_case`, mirroring ObjC selector semantics.
- `NSUInteger → usize`, `NSRange` kept for ranges; pointer safety documented on methods.
- Re-export modules cleanly from `lib.rs` for `use crate::…` ergonomics.

### Compilation fixes
- Added base `command_encoder` traits and flags to satisfy encoder hierarchies.
- Bridged missing protocols (`ComputePipelineState`, `IndirectCommandBuffer`, `CounterSampleBuffer`).
- Rewired `ComputeCommandEncoder` to use bridged types and added missing selectors.
- Re-exported `FunctionHandle` from `visible_function_table` for intersection table.

### Next candidates (not yet ported here)
- Render pass/pipeline and large render command encoder surface.
- Remaining `MTL*` files in root (e.g., where not yet explicitly removed).

### Notes
- All feature-gates were dropped in ported modules.
- Where upstream types aren’t fully modeled yet, minimal bridges were added to get clean builds while preserving API shape for subsequent fleshing out.

