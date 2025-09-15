use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

extern_class!(
    /// Listener for SharedEvent notifications from Metal.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSharedEventListener;
);

unsafe impl Send for MTLSharedEventListener {}
unsafe impl Sync for MTLSharedEventListener {}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLSharedEventListener {}
);

impl MTLSharedEventListener {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        // dispatch queue helpers omitted in this port

        #[unsafe(method(sharedListener))]
        #[unsafe(method_family = none)]
        pub unsafe fn shared_listener() -> Retained<MTLSharedEventListener>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLSharedEventListener {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
