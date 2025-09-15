use objc2::{Encode, Encoding, RefEncode};

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LanguageVersion {
    Version1_1 = (1 << 16) + 1,
    Version1_2 = (1 << 16) + 2,
    Version2_0 = 2 << 16,
    Version2_1 = (2 << 16) + 1,
    Version2_2 = (2 << 16) + 2,
    Version2_3 = (2 << 16) + 3,
    Version2_4 = (2 << 16) + 4,
    Version3_0 = (3 << 16) + 0,
    Version3_1 = (3 << 16) + 1,
    Version3_2 = (3 << 16) + 2,
    Version4_0 = (4 << 16) + 0,
}

unsafe impl Encode for LanguageVersion {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for LanguageVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LibraryType {
    Executable = 0,
    Dynamic = 1,
}
unsafe impl Encode for LibraryType {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for LibraryType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LibraryOptimizationLevel {
    Default = 0,
    Size = 1,
}
unsafe impl Encode for LibraryOptimizationLevel {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for LibraryOptimizationLevel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompileSymbolVisibility {
    Default = 0,
    Hidden = 1,
}
unsafe impl Encode for CompileSymbolVisibility {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for CompileSymbolVisibility {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MathMode {
    Safe = 0,
    Relaxed = 1,
    Fast = 2,
}
unsafe impl Encode for MathMode {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MathMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MathFloatingPointFunctions {
    Fast = 0,
    Precise = 1,
}
unsafe impl Encode for MathFloatingPointFunctions {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MathFloatingPointFunctions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LibraryError {
    Unsupported = 1,
    Internal = 2,
    CompileFailure = 3,
    CompileWarning = 4,
    FunctionNotFound = 5,
    FileNotFound = 6,
}
unsafe impl Encode for LibraryError {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for LibraryError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
