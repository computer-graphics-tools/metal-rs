use objc2::{Encode, Encoding, RefEncode};

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MLTLanguageVersion {
    Version1_1 = (1 << 16) + 1,
    Version1_2 = (1 << 16) + 2,
    Version2_0 = 2 << 16,
    Version2_1 = (2 << 16) + 1,
    Version2_2 = (2 << 16) + 2,
    Version2_3 = (2 << 16) + 3,
    Version2_4 = (2 << 16) + 4,
    Version3_0 = (3 << 16),
    Version3_1 = (3 << 16) + 1,
    Version3_2 = (3 << 16) + 2,
    Version4_0 = (4 << 16),
}

unsafe impl Encode for MLTLanguageVersion {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MLTLanguageVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLLibraryType {
    Executable = 0,
    Dynamic = 1,
}
unsafe impl Encode for MTLLibraryType {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLLibraryType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLLibraryOptimizationLevel {
    Default = 0,
    Size = 1,
}
unsafe impl Encode for MTLLibraryOptimizationLevel {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLLibraryOptimizationLevel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCompileSymbolVisibility {
    Default = 0,
    Hidden = 1,
}
unsafe impl Encode for MTLCompileSymbolVisibility {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLCompileSymbolVisibility {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLMathMode {
    Safe = 0,
    Relaxed = 1,
    Fast = 2,
}
unsafe impl Encode for MTLMathMode {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLMathMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLMathFloatingPointFunctions {
    Fast = 0,
    Precise = 1,
}
unsafe impl Encode for MTLMathFloatingPointFunctions {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLMathFloatingPointFunctions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLLibraryError {
    Unsupported = 1,
    Internal = 2,
    CompileFailure = 3,
    CompileWarning = 4,
    FunctionNotFound = 5,
    FileNotFound = 6,
}
unsafe impl Encode for MTLLibraryError {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLLibraryError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
