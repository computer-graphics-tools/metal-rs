mod drawable;
mod presented_handler;

pub use drawable::{MTLDrawable, MTLDrawableExt};
pub use presented_handler::MTLDrawablePresentedHandler;

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    use objc2::runtime::ProtocolObject;
    use objc2_foundation::NSObjectProtocol;

    use super::{MTLDrawable, MTLDrawableExt, MTLDrawablePresentedHandler};

    fn assert_nsobject<T: NSObjectProtocol + ?Sized>() {}

    fn register_presented_handler(
        drawable: &ProtocolObject<dyn MTLDrawable>,
        handler: &MTLDrawablePresentedHandler,
    ) {
        drawable.add_presented_handler(handler);
    }

    #[test]
    fn protocol_inheritance_and_presented_handler_api_match_the_header() {
        assert_nsobject::<dyn MTLDrawable>();

        let _: fn(&ProtocolObject<dyn MTLDrawable>, &MTLDrawablePresentedHandler) = register_presented_handler;
    }

    #[test]
    fn presented_handler_accepts_sendable_thread_safe_captures() {
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = Arc::clone(&calls);
        let _handler = MTLDrawablePresentedHandler::new(move |_| {
            callback_calls.fetch_add(1, Ordering::Relaxed);
        });

        assert_eq!(calls.load(Ordering::Relaxed), 0);
    }
}
