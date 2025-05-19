# Porting Apple's Metal.framework to Rust (`metal-rs`)

> **Goal** – Transform the entirety of Apple's Objective-C Metal SDK into an idiomatic, memory-safe Rust library that shares the same design philosophy and dependencies as `mpsgraph-rs` (namely **`objc2`**, **`objc2-foundation`**, and **`block2`**).  This document distils the architecture of the SDK, lessons from Apple's *metal-cpp* headers, and concrete guidelines for carrying the port to completion.

---

## 0. Table of Contents

1. Terminology & source materials  
2. High-level comparison: Obj-C vs C++ vs Rust  
3. Crate layout & build script  
4. Dependency primer (objc2, foundation, block2)  
5. Mapping headers → Rust modules  
6. The porting recipe (10 repeatable steps)  
7. Memory-management patterns (`Retained<T>`)  
8. Enums & option sets (`bitflags!`)  
9. Dealing with Objective-C **blocks**  
10. Testing & CI  
11. Automation opportunities  
12. FAQ & further reading  

---

## 1. Terminology & Source Materials

| Abbrev. | Meaning | Location |
|---------|---------|----------|
| *SDK* | The public Objective-C headers shipped in **`Xcode.app/…/Metal.framework/Headers`** | Attached as `Metal.framework` |
| *metal-cpp* | Apple-maintained C++11 wrappers that translate every Obj-C selector into a C++ method | Attached as `metal-cpp/Metal/*.hpp` |
| *mpsgraph-rs* | Finished Rust bindings for the Metal Performance Shaders *Graph* framework that already rely on `objc2` | `mpsgraph-rs/` sub-workspace |
| *metal-rs* | **This** crate – work-in-progress rewrite that will cover the whole Metal (+MetalFX, +QuartzCore) surface | Repository root |

All three artefacts expose nearly identical APIs; therefore we can treat *metal-cpp* as a reference implementation when the Obj-C documentation lacks detail.

---

## 2. High-level Comparison: Obj-C → C++ → Rust

| Design choice | Objective-C SDK | metal-cpp | Target Rust style |
|---------------|-----------------|-----------|-------------------|
| Object identity | `id<MTLDevice>` / class pointers | `MTL::Device*` smart-ptr | `Retained<Device>` / `&Device` |
| Reference counting | ARC (+0 / +1 rules) | `NS::RetainPtr` | `objc2::rc::Retained<T>` |
| Error handling | `NSError **` out-params | `NS::Error` exceptions | `Result<T, Error>` or `Option<T>` |
| Async callbacks | Obj-C **blocks** | `std::function` wrappers | `block2::ConcreteBlock` |
| Option sets | `NS_OPTIONS` macro | `namespace MTL::ResourceOptions` enum | `bitflags!` |
| Enums | `typedef NS_ENUM` | `enum class` | `#[repr(u64)] enum` |
| Namespacing | Prefix `MTL` | C++ `MTL::` namespace | Rust `metal::` module |

The Rust port keeps *exact numeric values* for enums/flags (important for ABI) while stripping the `MTL` prefix where the surrounding module already clarifies the domain (`metal::Device`).

---

## 3. Crate Layout & Build Script

```
metal-rs/
├── build.rs                     # single point that links system frameworks
├── Cargo.toml                   # depends on objc2 / foundation / block2
├── src/
│   ├── lib.rs                  # public re-exports, feature-flags
│   ├── prelude.rs              # common use declarations + Retained
│   ├── device.rs               # metal::Device (protocol)
│   ├── command_queue.rs        # metal::CommandQueue (protocol)
│   ├── buffer.rs               # metal::Buffer (protocol)
│   ├── … one file per Obj-C header …
├── PORTING_METAL_TO_RUST_GUIDE.md   # <-- this document
└── TODO_PORTING_STATUS.md      # running progress checklist
```

**build.rs** example:

The `build.rs` script is responsible for linking the necessary system frameworks. It should unconditionally link frameworks like `Metal` and `QuartzCore`, as the library aims for platform-agnostic compilation where possible. The actual availability and functionality of Metal will be determined at runtime by the target system.

```rust
fn main() {
    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=framework=QuartzCore");
    // Link other frameworks like MetalPerformanceShaders if they are directly used.
    // e.g., println!("cargo:rustc-link-lib=framework=MetalPerformanceShaders");
}
```

The removal of `#[cfg(any(target_os = "macos", target_os = "ios"))]` attributes from the library source code and build scripts aligns with this approach, simplifying the build process and deferring platform compatibility checks to the linkage and runtime phases. If a feature is truly only available on certain OS versions, this can be handled with more granular `cfg` attributes or runtime checks within the relevant functions, rather than broad, file-level exclusions.

---

## 4. Dependency Primer

* **`objc2`** – runtime glue, `extern_class!`, `extern_protocol!`, `msg_send!`, `Retained<T>`, `autoreleasepool`.
* **`objc2-foundation`** – safe wrappers around `NSString`, `NSArray`, `NSError`, …
* **`block2`** – bridges Obj-C blocks (`^ { … }`) to Rust closures via `ConcreteBlock`.
* **`bitflags`** – declares transparent wrappers for `NS_OPTIONS` groups.
* **`once_cell`** – caches selectors / classes when a per-call lookup would be wasteful.

The versions must stay **in lock-step** with `mpsgraph-rs` to guarantee that an application can use both crates simultaneously without duplicate symbols.

---

## 5. Mapping Headers → Rust Modules

| Header | Kind | Rust file | Status |
|--------|------|-----------|--------|
| `MTLDevice.h` | *protocol* | `device.rs` | **done (MVP)** |
| `MTLCommandQueue.h` | protocol | `command_queue.rs` | ☐ |
| `MTLCommandBuffer.h` | protocol | `command_buffer.rs` | ☐ |
| `MTLBuffer.h` | protocol | `buffer.rs` | ☐ |
| `MTLTexture.h` | protocol | `texture.rs` | ☐ |
| `MTLTextureDescriptor.h` | class | `texture_descriptor.rs` | ☐ |
| … *(~60 additional headers)* | … | … | … |

The above table is duplicated and auto-generated in **`TODO_PORTING_STATUS.md`** so contributors can tick checkboxes.

---

## 6. The Porting Recipe (Repeat per header)

1. **Identify category** – `@interface` = class, `@protocol` = protocol.
2. **Create file** in `src/` with snake-case version of the name.
3. **Add wrapper skeleton**:
   ```rust
   use crate::prelude::*;
   extern_protocol!(unsafe trait CommandQueueProtocol: NSObjectProtocol);
   pub type CommandQueue = objc2::runtime::ProtocolObject<dyn CommandQueueProtocol>;
   ```
4. **Translate each selector** to a Rust method; mind ownership semantics (+0 vs +1).
5. **Convert enums / option sets** (see §8 below).
6. **Handle out-parameters / `NSError**`** via `Result`.
7. **Build example snippet in doc-comment** (`rust, no_run`) that covers at least one happy path.
8. **Run tests**: `cargo test --doc` under an `autoreleasepool`.
9. **Clippy + rustfmt**.
10. **Update `TODO_PORTING_STATUS.md`** marking the header as *done*.

---

## 7. Memory-Management Patterns

| Objective-C return | Meaning | Rust call |
|--------------------|---------|-----------|
| *+0* (autoreleased) | must retain | `Retained::retain_autoreleased(ptr)` |
| *+1* (`alloc/init`, `copy`, `new`) | already retained | `Retained::from_raw(ptr).unwrap()` |

Always hide raw pointers behind `Retained<T>` in public APIs. Borrowed access uses `&T` / `&mut T` just like normal Rust values.  Use `autoreleasepool(|| { … })` in performance-critical loops.

---

## 8. Enums & Option Sets

### 8.1 Plain enums

```rust
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[doc(alias = "MTLFeatureSet")]
pub enum FeatureSet {
    MacOS_GPUFamily1_v4 = 10004,
    // …
}
```

### 8.2 `NS_OPTIONS` groups

```rust
bitflags::bitflags! {
    #[repr(transparent)]
    #[doc(alias = "MTLResourceOptions")]
    pub struct ResourceOptions: u64 {
        const CPUCacheModeDefaultCache  = 1 << 0;
        const StorageModeShared         = 1 << 4;
        // …
    }
}
```

---

## 9. Dealing with Objective-C **Blocks**

```rust
use block2::ConcreteBlock;

let finished = ConcreteBlock::new(|cmd_buf: &CommandBuffer| {
    println!("GPU start = {}", cmd_buf.gpu_start_time());
}).copy();
unsafe { msg_send![cmd_buf, addCompletedHandler: &*finished]; }
```

Rules:
1. Always call `.copy()` so the Obj-C runtime retains the block.  
2. Move any captured `Retained<T>` into the closure to extend lifetime.  
3. Do **not** block on GPU work inside the callback – prefer async Rust channel.

---

## 10. Testing & CI

* The workspace already ships a GitHub Action (*macOS-13*) running `cargo clippy -- -D warnings && cargo test --workspace`.
* Integration tests are located under `tests/` and automatically skip on non-Apple targets.
* Use `#[test]` + `autoreleasepool` to avoid leaks.

---

## 11. Automation Opportunities

* A **header-parser** (Swift or Rust) can emit Rust `extern_protocol!` stubs and `bitflags!` definitions by inspecting `*.apinotes` or the C headers directly via Clang.
* Use **bindgen** only for constant extraction – do *not* expose `unsafe extern "C"` calls.
* Synchronise enum/flag numeric values with `metal-cpp`  to detect divergence.

---

## 12. FAQ & Further Reading

**Q:** *Why `objc2` over the older `objc` crate?*  
**A:** `objc2` offers zero-cost message sending, `Retained<T>` ownership tracking, and compile-time selector caching.

**Q:** *What about MetalFX / Vision / Core Animation?*  
**A:** Same recipe applies; consider separate crates that depend on `metal-rs` for shared types.

**Q:** *Can we keep the old API around behind a feature flag?*  
**A:** Not worth the maintenance burden – migrate examples and publish **0.2.0** as the first ObjC2-based release.

**Further reading**:  
* Apple – *Using Metal in Swift*  
* Apple – *Metal Shading Language Guide*  
* [`mpsgraph-rs` source](mpsgraph-rs/crates/mpsgraph-rs)  
* [`objc2` docs](https://docs.rs/objc2)  
* [`block2` docs](https://docs.rs/block2)

---

Happy hacking – *and may all your command buffers return `MTLCommandBufferStatusCompleted`!* 