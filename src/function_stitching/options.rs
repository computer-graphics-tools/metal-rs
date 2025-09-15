use objc2::{Encode, Encoding, RefEncode};

/// Options to create a stitched library (from `MTLStitchedLibraryOptions`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StitchedLibraryOptions(pub usize);
bitflags::bitflags! {
    impl StitchedLibraryOptions: usize {
        const None = 0;
        const FailOnBinaryArchiveMiss = 1<<0;
        const StoreLibraryInMetalPipelinesScript = 1<<1;
    }
}

unsafe impl Encode for StitchedLibraryOptions {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for StitchedLibraryOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
