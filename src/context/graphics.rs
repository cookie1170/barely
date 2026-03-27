use crate::graphics::utils::{self, RenderPassBuilder};
use crate::prelude::Color;

impl<'a> super::Context<'a> {
    pub fn clear_screen(&mut self, color: Color) {
        let attachments = [Some(utils::op_attachment(
            utils::store(wgpu::LoadOp::Clear(color.into())),
            self.view,
        ))];

        let pass = RenderPassBuilder::new(Some("Clear screen")).color_attachments(&attachments);

        self.encoder.begin_render_pass(&pass.into());
    }
}
