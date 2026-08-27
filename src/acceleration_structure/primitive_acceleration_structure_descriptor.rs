use core::ffi::c_float;

use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol};

use crate::{MTLAccelerationStructureDescriptor, MTLAccelerationStructureGeometryDescriptor, MTLMotionBorderMode};

extern_class!(
    /// Descriptor for a primitive acceleration structure
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor?language=objc)
    #[unsafe(super(MTLAccelerationStructureDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLPrimitiveAccelerationStructureDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLPrimitiveAccelerationStructureDescriptor {}
);

unsafe impl CopyingHelper for MTLPrimitiveAccelerationStructureDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLPrimitiveAccelerationStructureDescriptor {}
);

impl MTLPrimitiveAccelerationStructureDescriptor {
    extern_methods!(
        /// Motion border mode describing what happens if acceleration structure is sampled before
        /// motionStartTime. If not set defaults to MTLMotionBorderModeClamp.
        #[unsafe(method(motionStartBorderMode))]
        #[unsafe(method_family = none)]
        pub fn motion_start_border_mode(&self) -> MTLMotionBorderMode;

        /// Setter for [`motionStartBorderMode`][Self::motionStartBorderMode].
        #[unsafe(method(setMotionStartBorderMode:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_start_border_mode(
            &self,
            motion_start_border_mode: MTLMotionBorderMode,
        );

        /// Motion border mode describing what happens if acceleration structure is sampled after
        /// motionEndTime. If not set defaults to MTLMotionBorderModeClamp.
        #[unsafe(method(motionEndBorderMode))]
        #[unsafe(method_family = none)]
        pub fn motion_end_border_mode(&self) -> MTLMotionBorderMode;

        /// Setter for [`motionEndBorderMode`][Self::motionEndBorderMode].
        #[unsafe(method(setMotionEndBorderMode:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_end_border_mode(
            &self,
            motion_end_border_mode: MTLMotionBorderMode,
        );

        /// Motion start time of this geometry. If not set defaults to 0.0f.
        #[unsafe(method(motionStartTime))]
        #[unsafe(method_family = none)]
        pub fn motion_start_time(&self) -> c_float;

        /// Setter for [`motionStartTime`][Self::motionStartTime].
        #[unsafe(method(setMotionStartTime:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_start_time(
            &self,
            motion_start_time: c_float,
        );

        /// Motion end time of this geometry. If not set defaults to 1.0f.
        #[unsafe(method(motionEndTime))]
        #[unsafe(method_family = none)]
        pub fn motion_end_time(&self) -> c_float;

        /// Setter for [`motionEndTime`][Self::motionEndTime].
        #[unsafe(method(setMotionEndTime:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_end_time(
            &self,
            motion_end_time: c_float,
        );

        /// Motion keyframe count. Is 1 by default which means no motion.
        #[unsafe(method(motionKeyframeCount))]
        #[unsafe(method_family = none)]
        pub fn motion_keyframe_count(&self) -> usize;

        /// Setter for [`motionKeyframeCount`][Self::motionKeyframeCount].
        #[unsafe(method(setMotionKeyframeCount:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_keyframe_count(
            &self,
            motion_keyframe_count: usize,
        );

        #[unsafe(method(descriptor))]
        #[unsafe(method_family = none)]
        pub fn descriptor() -> Retained<Self>;
    );

    /// Geometry descriptors used to build this acceleration structure.
    ///
    /// When `motion_keyframe_count` is greater than one, every descriptor must
    /// describe motion geometry with the same number of primitive-buffer keyframes.
    pub fn geometry_descriptors(&self) -> Option<Box<[Retained<MTLAccelerationStructureGeometryDescriptor>]>> {
        let descriptors: Option<Retained<NSArray<MTLAccelerationStructureGeometryDescriptor>>> =
            unsafe { msg_send![self, geometryDescriptors] };
        descriptors.map(|descriptors| descriptors.to_vec().into_boxed_slice())
    }

    /// Sets the geometry descriptors used to build this acceleration structure.
    pub fn set_geometry_descriptors(
        &self,
        geometry_descriptors: Option<&[&MTLAccelerationStructureGeometryDescriptor]>,
    ) {
        let descriptors = geometry_descriptors.map(NSArray::from_slice);
        unsafe {
            let _: () = msg_send![self, setGeometryDescriptors: descriptors.as_deref()];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTLPrimitiveAccelerationStructureDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
