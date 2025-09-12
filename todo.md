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

## Per-header checklist

### Metal.apinotes
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### Metal.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4AccelerationStructure.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4Archive.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4ArgumentTable.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4BinaryFunction.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4BinaryFunctionDescriptor.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4BufferRange.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4CommandAllocator.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4CommandBuffer.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4CommandEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4CommandQueue.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4CommitFeedback.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4Compiler.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4CompilerTask.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4ComputeCommandEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4ComputePipeline.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4Counters.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4FunctionDescriptor.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4LibraryDescriptor.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4LibraryFunctionDescriptor.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4LinkingDescriptor.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4MachineLearningCommandEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4MachineLearningPipeline.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4MeshRenderPipeline.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4PipelineDataSetSerializer.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4PipelineState.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4RenderCommandEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4RenderPass.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4RenderPipeline.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4SpecializedFunctionDescriptor.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4StitchedFunctionDescriptor.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTL4TileRenderPipeline.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLAccelerationStructure.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLAccelerationStructureCommandEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLAccelerationStructureTypes.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLAllocation.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLArgument.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [x] Enums/bitflags — BindingAccess
- [x] Classes — ArgumentDescriptor
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLArgumentEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLBinaryArchive.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLBlitCommandEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLBlitPass.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLBuffer.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLCaptureManager.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLCaptureScope.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLCommandBuffer.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLCommandEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLCommandQueue.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLComputeCommandEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLComputePass.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLComputePipeline.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLCounters.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLDataType.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [x] Enums/bitflags — DataType
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLDefines.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLDepthStencil.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLDevice.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [x] Enums/bitflags — IoCompressionMethod, FeatureSet, GpuFamily, DeviceLocation, PipelineOption, ReadWriteTextureTier, ArgumentBuffersTier, SparseTextureRegionAlignmentMode, CounterSamplingPoint
- [x] Structs — AccelerationStructureSizes, SizeAndAlign
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLDeviceCertification.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLDrawable.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLDynamicLibrary.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLEvent.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLFence.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLFunctionConstantValues.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLFunctionDescriptor.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLFunctionHandle.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLFunctionLog.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLFunctionStitching.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLGPUAddress.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLHeap.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLIndirectCommandBuffer.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLIndirectCommandEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLIntersectionFunctionTable.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLIOCommandBuffer.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLIOCommandQueue.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLIOCompressor.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLLibrary.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLLinkedFunctions.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLLogState.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLParallelRenderCommandEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLPipeline.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLPixelFormat.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLRasterizationRate.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLRenderCommandEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLRenderPass.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLRenderPipeline.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLResidencySet.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLResource.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLResourceStateCommandEncoder.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLResourceStatePass.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLResourceViewPool.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLSampler.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLStageInputOutputDescriptor.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLTensor.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLTexture.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [x] Enums/bitflags — TextureType
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLTextureViewPool.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLTypes.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [x] Structs — Origin, Size, Region, SamplePosition, Coordinate2D, ResourceId
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLVertexDescriptor.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

### MTLVisibleFunctionTable.h
- [ ] Inventory header
- [ ] Design Rust API
- [ ] Low-level bindings
- [ ] Enums/bitflags
- [ ] Methods/properties
- [ ] Availability/feature gates
- [ ] Safety audit
- [ ] Tests/examples
- [ ] Docs

