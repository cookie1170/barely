use crate::graphics::buffer::GetBuffer;
use crate::graphics::utils::{self, RenderPassBuilder};
use crate::prelude::*;

impl Context<'_> {
    pub fn clear_screen(&mut self, color: Color) {
        let attachments =
            utils::op_attachments(utils::store(wgpu::LoadOp::Clear(color.into())), self.view);

        RenderPassBuilder::new("Clear screen")
            .color_attachments(&attachments)
            .begin(self.encoder);
    }

    pub fn draw_vertices_unindexed<M: Material>(
        &mut self,
        buffer: &impl GetBuffer,
        material: &MaterialHandle<M>,
    ) {
        let mut pass = RenderPassBuilder::load("Draw vertices unindexed", self.view, self.encoder);

        let pipeline = material.get_pipeline();

        pass.set_pipeline(pipeline);
        pass.set_vertex_buffer(0, buffer.get_buffer(self.handle).slice(..));
        pass.set_bind_group(0, &self.handle.camera_bind_group, &[]);
        material.update_pass(&mut pass);
        pass.draw(0..buffer.get_length(), 0..1);
    }

    pub fn draw_vertices<M: Material>(
        &mut self,
        vertex_buffer: &impl GetBuffer,
        index_buffer: &impl GetBuffer,
        material: &MaterialHandle<M>,
    ) {
        let mut pass = RenderPassBuilder::load("Draw vertices", self.view, self.encoder);

        let pipeline = material.get_pipeline();
        pass.set_pipeline(pipeline);
        pass.set_vertex_buffer(0, vertex_buffer.get_buffer(self.handle).slice(..));
        pass.set_index_buffer(
            index_buffer.get_buffer(self.handle).slice(..),
            wgpu::IndexFormat::Uint32,
        );
        material.update_pass(&mut pass);
        pass.draw_indexed(0..index_buffer.get_length(), 0, 0..1);
    }
}
