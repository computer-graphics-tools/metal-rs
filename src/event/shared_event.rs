use core::ptr::NonNull;

use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSObjectProtocol;

use super::Event;
use super::{SharedEventHandle, SharedEventListener};

pub type SharedEventNotificationBlock =
    *mut block2::DynBlock<dyn Fn(NonNull<ProtocolObject<dyn SharedEvent>>, u64)>;

extern_protocol!(
    /// Shared event that can be signaled and waited on across devices.
    #[name = "MTLSharedEvent"]
    pub unsafe trait SharedEvent: Event {
        /// Register a callback for when the event reaches a value.
        #[unsafe(method(notifyListener:atValue:block:))]
        #[unsafe(method_family = none)]
        unsafe fn notify_listener_at_value_block(
            &self,
            listener: &SharedEventListener,
            value: u64,
            block: SharedEventNotificationBlock,
        );

        #[unsafe(method(newSharedEventHandle))]
        #[unsafe(method_family = new)]
        unsafe fn new_shared_event_handle(&self) -> Retained<SharedEventHandle>;

        #[unsafe(method(waitUntilSignaledValue:timeoutMS:))]
        #[unsafe(method_family = none)]
        unsafe fn wait_until_signaled_value_timeout_ms(
            &self,
            value: u64,
            milliseconds: u64,
        ) -> bool;

        #[unsafe(method(signaledValue))]
        #[unsafe(method_family = none)]
        unsafe fn signaled_value(&self) -> u64;

        /// Setter for [`signaled_value`][Self::signaled_value].
        #[unsafe(method(setSignaledValue:))]
        #[unsafe(method_family = none)]
        unsafe fn set_signaled_value(&self, signaled_value: u64);
    }
);
