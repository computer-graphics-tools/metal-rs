use objc2::{Encode, Encoding, RefEncode};

/// Describes how a resource will be used by a shader through an argument buffer (from `MTLResourceUsage`).
///
/// Availability: macOS 10.13+, iOS 11.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLResourceUsage(pub usize);
bitflags::bitflags! {
    impl MTLResourceUsage: usize {
        const Read = 1<<0;
        const Write = 1<<1;
        #[deprecated(note = "use MTLResourceUsage::Read")]
        const Sample = 1<<2;
    }
}

unsafe impl Encode for MTLResourceUsage {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLResourceUsage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes the types of resources that a barrier operates on (from `MTLBarrierScope`).
///
/// Availability: macOS 10.14+, iOS 12.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLBarrierScope(pub usize);
bitflags::bitflags! {
    impl MTLBarrierScope: usize {
        const Buffers = 1<<0;
        const Textures = 1<<1;
        /// Available on macOS and Mac Catalyst; unavailable on iOS.
        const RenderTargets = 1<<2;
    }
}

unsafe impl Encode for MTLBarrierScope {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLBarrierScope {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes stages of GPU work (from `MTLStages`).
///
/// Availability: macOS 26.0+, iOS 26.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLStages(pub usize);
bitflags::bitflags! {
    impl MTLStages: usize {
        const Vertex = 1<<0;
        const Fragment = 1<<1;
        const Tile = 1<<2;
        const Object = 1<<3;
        const Mesh = 1<<4;
        const ResourceState = 1<<26;
        const Dispatch = 1<<27;
        const Blit = 1<<28;
        const AccelerationStructure = 1<<29;
        const MachineLearning = 1<<30;
        const All = isize::MAX as usize;
    }
}

unsafe impl Encode for MTLStages {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLStages {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn option_sets_match_platform_nsuinteger_abi() {
        assert_eq!(size_of::<MTLResourceUsage>(), size_of::<usize>());
        assert_eq!(align_of::<MTLResourceUsage>(), align_of::<usize>());
        assert_eq!(size_of::<MTLBarrierScope>(), size_of::<usize>());
        assert_eq!(align_of::<MTLBarrierScope>(), align_of::<usize>());
        assert_eq!(size_of::<MTLStages>(), size_of::<usize>());
        assert_eq!(align_of::<MTLStages>(), align_of::<usize>());
    }

    #[test]
    #[allow(deprecated)]
    fn resource_usage_and_barrier_scope_bits_match_metal_values() {
        assert_eq!(MTLResourceUsage::Read.bits(), 1 << 0);
        assert_eq!(MTLResourceUsage::Write.bits(), 1 << 1);
        assert_eq!(MTLResourceUsage::Sample.bits(), 1 << 2);
        assert_eq!(MTLBarrierScope::Buffers.bits(), 1 << 0);
        assert_eq!(MTLBarrierScope::Textures.bits(), 1 << 1);
        assert_eq!(MTLBarrierScope::RenderTargets.bits(), 1 << 2);
    }

    #[test]
    fn stage_masks_match_metal_values() {
        assert_eq!(MTLStages::Vertex.bits(), 1 << 0);
        assert_eq!(MTLStages::Fragment.bits(), 1 << 1);
        assert_eq!(MTLStages::Tile.bits(), 1 << 2);
        assert_eq!(MTLStages::Object.bits(), 1 << 3);
        assert_eq!(MTLStages::Mesh.bits(), 1 << 4);
        assert_eq!(MTLStages::ResourceState.bits(), 1 << 26);
        assert_eq!(MTLStages::Dispatch.bits(), 1 << 27);
        assert_eq!(MTLStages::Blit.bits(), 1 << 28);
        assert_eq!(MTLStages::AccelerationStructure.bits(), 1 << 29);
        assert_eq!(MTLStages::MachineLearning.bits(), 1 << 30);
        assert_eq!(MTLStages::All.bits(), isize::MAX as usize);
    }
}
