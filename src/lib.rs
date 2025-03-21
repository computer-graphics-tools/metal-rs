// Suppress unexpected_cfgs warnings from objc macros
#![allow(unexpected_cfgs)]

// Helper macros
#[macro_export]
macro_rules! foreign_obj_type {
    (type CType = $raw_ident:ty; pub struct $owned:ident; pub struct $ref_ident:ident; type ParentType = $parent:ty;) => {
        pub enum $ref_ident {}

        unsafe impl ::foreign_types::ForeignTypeRef for $ref_ident {
            type CType = $raw_ident;
        }

        unsafe impl ::std::marker::Send for $ref_ident {}
        unsafe impl ::std::marker::Sync for $ref_ident {}

        pub struct $owned(::foreign_types::ForeignObjectRef<$ref_ident>);

        unsafe impl ::foreign_types::ForeignType for $owned {
            type CType = $raw_ident;
            type Ref = $ref_ident;

            unsafe fn from_ptr(ptr: *mut Self::CType) -> Self {
                $owned(::foreign_types::ForeignObjectRef::from_ptr(ptr))
            }

            fn as_ptr(&self) -> *mut Self::CType {
                self.0.as_ptr()
            }
        }

        impl ::std::ops::Deref for $owned {
            type Target = $ref_ident;

            fn deref(&self) -> &$ref_ident {
                &*self.0
            }
        }

        impl ::std::borrow::Borrow<$ref_ident> for $owned {
            fn borrow(&self) -> &$ref_ident {
                &*self.0
            }
        }

        impl ::std::clone::Clone for $owned {
            fn clone(&self) -> $owned {
                unsafe {
                    let ptr = ::objc::msg_send![self.as_ref(), retain];
                    $owned::from_ptr(ptr)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! msg_send_bool {
    ($obj:expr, $selector:ident) => {
        {
            let result: ::objc::runtime::BOOL = msg_send![$obj, $selector];
            result != 0
        }
    };
    ($obj:expr, $selector:ident, $($args:expr),+) => {
        {
            let result: ::objc::runtime::BOOL = msg_send![$obj, $selector, $($args),+];
            result != 0
        }
    };
}

// Re-exports
pub mod foundation;
pub mod metal;
pub mod quartzcore;