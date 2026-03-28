use crate::graphics::buffer::GetBuffer;
use crate::graphics::utils::{self, RenderPassBuilder};
use crate::prelude::*;

impl<'a> Context<'a> {
    pub fn clear_screen(&mut self, color: Color) {
        let attachments = [Some(utils::op_attachment(
            utils::store(wgpu::LoadOp::Clear(color.into())),
            self.view,
        ))];

        let pass = RenderPassBuilder::new(Some("Clear screen")).color_attachments(&attachments);

        self.encoder.begin_render_pass(&pass.into());
    }

    pub fn draw_vertex_buffer_unindexed<M: Material>(
        &mut self,
        buffer: &impl GetBuffer,
        material: &MaterialHandle<M>,
    ) {
        let attachments = [Some(utils::load_attachment(self.view))];
        let pass = RenderPassBuilder::new(Some("Draw vertex buffer unindexed"))
            .color_attachments(&attachments);

        let mut pass = self.encoder.begin_render_pass(&pass.into());
        let pipeline = material.get_pipeline();
        let length = buffer.get_length();
        let buffer = buffer.get_buffer(self.handle);

        pass.set_pipeline(pipeline);
        pass.set_vertex_buffer(0, buffer.slice(..));
        pass.draw(0..length, 0..1);
    }
}
