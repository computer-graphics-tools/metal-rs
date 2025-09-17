use objc2::{extern_class, extern_methods, rc::Retained, runtime::NSObject};

extern_class!(
    /// Reflection for a struct type used in argument/pipeline reflection.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStructType;
);

impl MTLStructType {
    extern_methods!(
        /// Array of members of this struct.
        ///
        /// Matches the `members` property in the header.
        #[unsafe(method(members))]
        #[unsafe(method_family = none)]
        pub fn members(
            &self,
        ) -> Retained<objc2_foundation::NSArray<crate::argument::MTLStructMember>>;

        /// Lookup a member by name.
        ///
        /// Returns `None` if no member with the given name exists.
        #[unsafe(method(memberByName:))]
        #[unsafe(method_family = none)]
        pub fn member_by_name(
            &self,
            name: &objc2_foundation::NSString,
        ) -> Option<Retained<crate::argument::MTLStructMember>>;
    );
}
