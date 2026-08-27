use core::ptr::NonNull;

use block2::RcBlock;
use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};

use super::{MTLEvent, MTLSharedEventHandle, MTLSharedEventListener};

/// A sendable block used for shared-event notifications.
pub struct MTLSharedEventNotificationBlock(RcBlock<dyn Fn(NonNull<ProtocolObject<dyn MTLSharedEvent>>, u64)>);

impl MTLSharedEventNotificationBlock {
    /// Creates a shared-event callback.
    ///
    /// Metal invokes this block on the listener's dispatch queue, so captured
    /// state must be safe to transfer and share between threads.
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&ProtocolObject<dyn MTLSharedEvent>, u64) + Send + Sync + 'static,
    {
        Self(RcBlock::new(move |event: NonNull<ProtocolObject<dyn MTLSharedEvent>>, value| {
            handler(unsafe { event.as_ref() }, value);
        }))
    }
}

/// Backwards-compatible name for [`MTLSharedEventNotificationBlock`].
pub type SharedEventNotificationBlock = MTLSharedEventNotificationBlock;

extern_protocol!(
    /// Shared event that can be signaled and waited on across devices.
    ///
    /// Availability: macOS 10.14+, iOS 12.0+
    pub unsafe trait MTLSharedEvent: MTLEvent {
        /// Convenience method for creating a shared event handle that may be passed to other processes via XPC.
        #[unsafe(method(newSharedEventHandle))]
        #[unsafe(method_family = new)]
        fn new_shared_event_handle(&self) -> Retained<MTLSharedEventHandle>;

        /// Synchronously wait for the signaled value to be >= `value`, with a timeout in milliseconds.
        #[unsafe(method(waitUntilSignaledValue:timeoutMS:))]
        #[unsafe(method_family = none)]
        fn wait_until_signaled_value_timeout_ms(
            &self,
            value: u64,
            milliseconds: u64,
        ) -> bool;

        /// Read the current signaled value.
        #[unsafe(method(signaledValue))]
        #[unsafe(method_family = none)]
        fn signaled_value(&self) -> u64;

        /// Set the event's signaled value.
        #[unsafe(method(setSignaledValue:))]
        #[unsafe(method_family = none)]
        fn set_signaled_value(
            &self,
            signaled_value: u64,
        );
    }
);

/// Safe convenience methods for [`MTLSharedEvent`].
pub trait MTLSharedEventExt: MTLSharedEvent + Message {
    /// Invokes `block` on `listener`'s dispatch queue after the event reaches
    /// `value`.
    fn notify_listener_at_value(
        &self,
        listener: &MTLSharedEventListener,
        value: u64,
        block: &MTLSharedEventNotificationBlock,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![
                self,
                notifyListener: listener,
                atValue: value,
                block: &*block.0
            ];
        }
    }
}

impl<T: MTLSharedEvent + Message> MTLSharedEventExt for T {}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, atomic::AtomicU64};

    use super::MTLSharedEventNotificationBlock;

    #[test]
    fn notification_block_accepts_send_sync_captures() {
        let value = Arc::new(AtomicU64::new(0));
        let _block = MTLSharedEventNotificationBlock::new(move |_event, _value| {
            let _ = &value;
        });
    }
}
