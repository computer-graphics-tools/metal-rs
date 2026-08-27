mod parallel_render_command_encoder;

pub use parallel_render_command_encoder::MTLParallelRenderCommandEncoder;

#[cfg(test)]
mod tests {
    use objc2::{rc::Retained, runtime::ProtocolObject};

    use super::MTLParallelRenderCommandEncoder;
    use crate::{MTLCommandEncoder, MTLRenderCommandEncoder};

    fn assert_command_encoder<T: MTLCommandEncoder + ?Sized>() {}

    fn render_command_encoder(
        encoder: &ProtocolObject<dyn MTLParallelRenderCommandEncoder>
    ) -> Option<Retained<ProtocolObject<dyn MTLRenderCommandEncoder>>> {
        encoder.render_command_encoder()
    }

    #[test]
    fn protocol_inheritance_and_nullable_ownership_match_the_header() {
        assert_command_encoder::<dyn MTLParallelRenderCommandEncoder>();

        let _: fn(
            &ProtocolObject<dyn MTLParallelRenderCommandEncoder>,
        ) -> Option<Retained<ProtocolObject<dyn MTLRenderCommandEncoder>>> = render_command_encoder;
    }
}
