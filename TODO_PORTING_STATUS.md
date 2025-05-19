# metal-rs – Porting Progress Tracker

> *Tick a box when the corresponding Rust wrapper fully compiles **and** has at least one doc-test.*
>
> Legend:  
> - [x] **Done** – implemented & tested  
> - [/] **In progress** – file exists but lacks some methods/enums  
> - [ ] **Todo** – no Rust file yet

---

## Core GPU Abstractions

| Obj-C Header | Rust file | Status |
|--------------|-----------|--------|
| `MTLDevice.h` | `device.rs` | [x] |
| `MTLCommandQueue.h` | `command_queue.rs` | [x] |
| `MTLCommandBuffer.h` | `command_buffer.rs` | [/] |
| `MTLBuffer.h` | `buffer.rs` | [/] |
| `MTLTexture.h` | `texture.rs` | [ ] |
| `MTLTextureDescriptor.h` | `texture_descriptor.rs` | [ ] |
| `MTLLibrary.h` | `library.rs` | [ ] |
| `MTLCompileOptions.h` | `compile_options.rs` | [/] |

## Encoding & Pipelines

| Obj-C Header | Rust file | Status |
|--------------|-----------|--------|
| `MTLRenderPipelineDescriptor.h` | `render_pipeline_descriptor.rs` | [ ] |
| `MTLRenderPipelineState.h` | `render_pipeline_state.rs` | [ ] |
| `MTLComputePipelineDescriptor.h` | `compute_pipeline_descriptor.rs` | [ ] |
| `MTLComputePipelineState.h` | `compute_pipeline_state.rs` | [ ] |
| `MTLDepthStencilDescriptor.h` | `depth_stencil_descriptor.rs` | [ ] |
| `MTLDepthStencilState.h` | `depth_stencil_state.rs` | [ ] |

## Encoders (Command-time)

| Obj-C Header | Rust file | Status |
|--------------|-----------|--------|
| `MTLRenderCommandEncoder.h` | `render_command_encoder.rs` | [ ] |
| `MTLComputeCommandEncoder.h` | `compute_command_encoder.rs` | [ ] |
| `MTLBlitCommandEncoder.h` | `blit_command_encoder.rs` | [ ] |
| `MTLAccelerationStructureCommandEncoder.h` | `acc_structure_command_encoder.rs` | [ ] |

## Synchronisation & Timing

| Obj-C Header | Rust file | Status |
|--------------|-----------|--------|
| `MTLEvent.h` | `event.rs` | [ ] |
| `MTLFence.h` | `fence.rs` | [ ] |
| `MTLSharedEvent.h` | `shared_event.rs` | [ ] |
| `MTLCounterSampleBuffer.h` | `counter_sample_buffer.rs` | [ ] |

## Resource Heaps & Memory

| Obj-C Header | Rust file | Status |
|--------------|-----------|--------|
| `MTLHeap.h` | `heap.rs` | [ ] |
| `MTLArgumentEncoder.h` | `argument_encoder.rs` | [ ] |
| `MTLBinaryArchive.h` | `binary_archive.rs` | [ ] |

## Ray-tracing (Metal 3)

| Obj-C Header | Rust file | Status |
|--------------|-----------|--------|
| `MTLAccelerationStructure.h` | `acceleration_structure.rs` | [ ] |
| `MTLIntersectionFunctionTable.h` | `intersection_function_table.rs` | [ ] |
| `MTLVisibleFunctionTable.h` | `visible_function_table.rs` | [ ] |

## MetalFX (optional feature `metalfx`)

| Obj-C Header | Rust file | Status |
|--------------|-----------|--------|
| `MFXSpatialScaler.h` | `metalfx/spatial_scaler.rs` | [ ] |
| `MFXTemporalScaler.h` | `metalfx/temporal_scaler.rs` | [ ] |

## Utilities

| Task | Status |
|------|--------|
| Delete legacy `src/foundation/` module | [ ] |
| CI job "clippy + tests" passes | [/] |
| Publish `0.2.0-alpha` to crates.io | [ ] |

---

*Last updated:* <!-- YYYY-MM-DD, keep in UTC --> 