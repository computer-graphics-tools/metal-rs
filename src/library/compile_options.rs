use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSDictionary, NSObjectProtocol, NSString};

use super::{
    CompileSymbolVisibility, LanguageVersion, LibraryOptimizationLevel, LibraryType,
    MathFloatingPointFunctions, MathMode,
};
use crate::types::Size as MTLSize;

extern_class!(
    /// Options for compiling Metal libraries.
    #[unsafe(super(NSObject))]
    #[name = "MTLCompileOptions"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CompileOptions;
);

extern_conformance!(
    unsafe impl NSCopying for CompileOptions {}
);

unsafe impl CopyingHelper for CompileOptions {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CompileOptions {}
);

impl CompileOptions {
    extern_methods!(
        #[unsafe(method(preprocessorMacros))]
        #[unsafe(method_family = none)]
        pub fn preprocessor_macros(&self) -> Option<Retained<NSDictionary<NSString, NSObject>>>;

        #[unsafe(method(setPreprocessorMacros:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_preprocessor_macros(
            &self,
            macros: Option<&NSDictionary<NSString, NSObject>>,
        );

        #[unsafe(method(mathMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn math_mode(&self) -> MathMode;

        #[unsafe(method(setMathMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_math_mode(&self, math_mode: MathMode);

        #[unsafe(method(mathFloatingPointFunctions))]
        #[unsafe(method_family = none)]
        pub unsafe fn math_floating_point_functions(&self) -> MathFloatingPointFunctions;

        #[unsafe(method(setMathFloatingPointFunctions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_math_floating_point_functions(&self, val: MathFloatingPointFunctions);

        #[unsafe(method(languageVersion))]
        #[unsafe(method_family = none)]
        pub fn language_version(&self) -> LanguageVersion;

        #[unsafe(method(setLanguageVersion:))]
        #[unsafe(method_family = none)]
        pub fn set_language_version(&self, v: LanguageVersion);

        #[unsafe(method(libraryType))]
        #[unsafe(method_family = none)]
        pub fn library_type(&self) -> LibraryType;

        #[unsafe(method(setLibraryType:))]
        #[unsafe(method_family = none)]
        pub fn set_library_type(&self, v: LibraryType);

        #[unsafe(method(installName))]
        #[unsafe(method_family = none)]
        pub fn install_name(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(setInstallName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_install_name(&self, name: Option<&NSString>);

        #[unsafe(method(preserveInvariance))]
        #[unsafe(method_family = none)]
        pub fn preserve_invariance(&self) -> bool;

        #[unsafe(method(setPreserveInvariance:))]
        #[unsafe(method_family = none)]
        pub fn set_preserve_invariance(&self, v: bool);

        #[unsafe(method(optimizationLevel))]
        #[unsafe(method_family = none)]
        pub unsafe fn optimization_level(&self) -> LibraryOptimizationLevel;

        #[unsafe(method(setOptimizationLevel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_optimization_level(&self, v: LibraryOptimizationLevel);

        #[unsafe(method(compileSymbolVisibility))]
        #[unsafe(method_family = none)]
        pub unsafe fn compile_symbol_visibility(&self) -> CompileSymbolVisibility;

        #[unsafe(method(setCompileSymbolVisibility:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_compile_symbol_visibility(&self, v: CompileSymbolVisibility);

        #[unsafe(method(allowReferencingUndefinedSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn allow_referencing_undefined_symbols(&self) -> bool;

        #[unsafe(method(setAllowReferencingUndefinedSymbols:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_allow_referencing_undefined_symbols(&self, v: bool);

        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub unsafe fn max_total_threads_per_threadgroup(&self) -> usize;

        #[unsafe(method(setMaxTotalThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_max_total_threads_per_threadgroup(&self, v: usize);

        #[unsafe(method(requiredThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub unsafe fn required_threads_per_threadgroup(&self) -> MTLSize;

        #[unsafe(method(setRequiredThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_required_threads_per_threadgroup(&self, v: MTLSize);

        #[unsafe(method(enableLogging))]
        #[unsafe(method_family = none)]
        pub unsafe fn enable_logging(&self) -> bool;

        #[unsafe(method(setEnableLogging:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_enable_logging(&self, v: bool);
    );
}

impl CompileOptions {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
