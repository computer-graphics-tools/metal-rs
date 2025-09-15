use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSURL};

extern_class!(
    /// A class used to indicate how an archive should be created.
    #[unsafe(super(NSObject))]
    #[name = "MTLBinaryArchiveDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BinaryArchiveDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for BinaryArchiveDescriptor {}
);

unsafe impl CopyingHelper for BinaryArchiveDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for BinaryArchiveDescriptor {}
);

impl BinaryArchiveDescriptor {
    extern_methods!(
        /// The file URL from which to open a BinaryArchive, or nil to create an empty BinaryArchive.
        #[unsafe(method(url))]
        #[unsafe(method_family = none)]
        pub fn url(&self) -> Option<Retained<NSURL>>;

        /// Setter for `url`. Copied when set.
        #[unsafe(method(setUrl:))]
        #[unsafe(method_family = none)]
        pub fn set_url(&self, url: Option<&NSURL>);
    );
}

impl BinaryArchiveDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}


