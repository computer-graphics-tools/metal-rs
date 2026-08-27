mod clear_color;
mod load_action;
mod render_pass_attachment_descriptor;
mod render_pass_color_attachment_descriptor;
mod render_pass_color_attachment_descriptor_array;
mod render_pass_depth_attachment_descriptor;
mod render_pass_descriptor;
mod render_pass_sample_buffer_attachment_descriptor;
mod render_pass_sample_buffer_attachment_descriptor_array;
mod render_pass_stencil_attachment_descriptor;
mod store_action;
mod store_action_options;
mod visibility_result_type;

pub use clear_color::MTLClearColor;
pub use load_action::MTLLoadAction;
pub use render_pass_attachment_descriptor::MTLRenderPassAttachmentDescriptor;
pub use render_pass_color_attachment_descriptor::MTLRenderPassColorAttachmentDescriptor;
pub use render_pass_color_attachment_descriptor_array::MTLRenderPassColorAttachmentDescriptorArray;
#[allow(deprecated)]
pub use render_pass_depth_attachment_descriptor::{
    MTLMultisampleDepthResolveFilter, MTLRenderPassDepthAttachmentDescriptor, MultisampleDepthResolveFilter,
};
pub use render_pass_descriptor::MTLRenderPassDescriptor;
pub use render_pass_sample_buffer_attachment_descriptor::MTLRenderPassSampleBufferAttachmentDescriptor;
pub use render_pass_sample_buffer_attachment_descriptor_array::MTLRenderPassSampleBufferAttachmentDescriptorArray;
pub use render_pass_stencil_attachment_descriptor::{
    MTLMultisampleStencilResolveFilter, MTLRenderPassStencilAttachmentDescriptor,
};
pub use store_action::MTLStoreAction;
pub use store_action_options::MTLStoreActionOptions;
pub use visibility_result_type::MTLVisibilityResultType;

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use objc2_foundation::NSCopying;

    use super::*;
    use crate::MTLSamplePosition;

    fn assert_copying<T: NSCopying + ?Sized>() {}

    #[test]
    fn clear_color_make_equivalent_matches_the_c_layout() {
        let color = MTLClearColor::new(0.25, 0.5, 0.75, 1.0);

        assert_eq!(color.red, 0.25);
        assert_eq!(color.green, 0.5);
        assert_eq!(color.blue, 0.75);
        assert_eq!(color.alpha, 1.0);
        assert_eq!(size_of::<MTLClearColor>(), 4 * size_of::<f64>());
        assert_eq!(align_of::<MTLClearColor>(), align_of::<f64>());
    }

    #[test]
    fn enum_values_and_platform_integer_abi_match_the_header() {
        assert_eq!(MTLLoadAction::DontCare as u64, 0);
        assert_eq!(MTLLoadAction::Load as u64, 1);
        assert_eq!(MTLLoadAction::Clear as u64, 2);

        assert_eq!(MTLStoreAction::DontCare as u64, 0);
        assert_eq!(MTLStoreAction::Store as u64, 1);
        assert_eq!(MTLStoreAction::MultisampleResolve as u64, 2);
        assert_eq!(MTLStoreAction::StoreAndMultisampleResolve as u64, 3);
        assert_eq!(MTLStoreAction::Unknown as u64, 4);
        assert_eq!(MTLStoreAction::CustomSampleDepthStore as u64, 5);

        assert_eq!(MTLStoreActionOptions::None.bits(), 0);
        assert_eq!(MTLStoreActionOptions::CustomSamplePositions.bits(), 1);
        assert_eq!(MTLVisibilityResultType::Reset as i64, 0);
        assert_eq!(MTLVisibilityResultType::Accumulate as i64, 1);
        assert_eq!(MTLMultisampleDepthResolveFilter::Sample0 as u64, 0);
        assert_eq!(MTLMultisampleDepthResolveFilter::Min as u64, 1);
        assert_eq!(MTLMultisampleDepthResolveFilter::Max as u64, 2);
        assert_eq!(MTLMultisampleStencilResolveFilter::Sample0 as u64, 0);
        assert_eq!(MTLMultisampleStencilResolveFilter::DepthResolvedSample as u64, 1);

        assert_eq!(size_of::<MTLLoadAction>(), size_of::<usize>());
        assert_eq!(size_of::<MTLStoreAction>(), size_of::<usize>());
        assert_eq!(size_of::<MTLStoreActionOptions>(), size_of::<usize>());
        assert_eq!(size_of::<MTLVisibilityResultType>(), size_of::<isize>());
        assert_eq!(size_of::<MTLMultisampleDepthResolveFilter>(), size_of::<usize>());
        assert_eq!(size_of::<MTLMultisampleStencilResolveFilter>(), size_of::<usize>());
    }

    #[test]
    fn descriptor_conformances_and_sample_position_wrappers_are_public() {
        fn inherited_attachment_accessors(descriptor: &MTLRenderPassColorAttachmentDescriptor) {
            let _ = descriptor.texture();
            let _ = descriptor.resolve_texture();
        }

        assert_copying::<MTLRenderPassAttachmentDescriptor>();
        assert_copying::<MTLRenderPassColorAttachmentDescriptor>();
        assert_copying::<MTLRenderPassDepthAttachmentDescriptor>();
        assert_copying::<MTLRenderPassStencilAttachmentDescriptor>();
        assert_copying::<MTLRenderPassSampleBufferAttachmentDescriptor>();
        assert_copying::<MTLRenderPassDescriptor>();

        let _: fn(&MTLRenderPassDescriptor, &[MTLSamplePosition]) = MTLRenderPassDescriptor::set_sample_positions;
        let _: fn(&MTLRenderPassDescriptor, &mut [MTLSamplePosition]) -> usize =
            MTLRenderPassDescriptor::get_sample_positions;
        let _: fn(&MTLRenderPassColorAttachmentDescriptor) = inherited_attachment_accessors;
    }
}
