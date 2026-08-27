mod command_encoder;
mod types;

pub use command_encoder::{MTLCommandEncoder, MTLCommandEncoderExt};
pub use types::{MTLBarrierScope, MTLResourceUsage, MTLStages};

#[cfg(test)]
mod tests {
    use core::any::TypeId;

    use super::MTLStages;
    use crate::MTLRenderStages;

    #[test]
    fn generic_and_render_stage_masks_remain_distinct_types() {
        assert_ne!(TypeId::of::<MTLStages>(), TypeId::of::<MTLRenderStages>());
    }
}
