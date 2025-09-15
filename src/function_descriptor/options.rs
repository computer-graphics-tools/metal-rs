use objc2::{Encode, Encoding, RefEncode};

/// Options for creating a `Function` (from `MTLFunctionOptions`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLFunctionOptions(pub usize);
bitflags::bitflags! {
    impl MTLFunctionOptions: usize {
        const None = 0;
        /// Compiles the found function. This enables dynamic linking of this Function.
        /// Only supported for visible functions.
        const CompileToBinary = 1<<0;
        /// Stores and tracks this function in a Metal Pipelines Script (binary archives context).
        const StoreFunctionInMetalPipelinesScript = 1<<1;
        /// Function creation fails if a lookup binary archive is specified and the function not found.
        const FailOnBinaryArchiveMiss = 1<<2;
        /// Compiles the function to have its function handles return a constant ResourceID across pipelines.
        /// Requires CompileToBinary.
        const PipelineIndependent = 1<<3;
    }
}

unsafe impl Encode for MTLFunctionOptions {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLFunctionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
