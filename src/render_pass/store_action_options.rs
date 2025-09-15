use objc2::{Encode, Encoding, RefEncode};

/// Options for store action (from `MTLStoreActionOptions`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StoreActionOptions(pub u64);

bitflags::bitflags! {
    impl StoreActionOptions: u64 {
        const None = 0;
        const CustomSamplePositions = 1<<0;
    }
}

unsafe impl Encode for StoreActionOptions {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for StoreActionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
