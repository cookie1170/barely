use std::num::NonZero;

pub struct RenderPassBuilder<'a> {
    label: Option<&'static str>,
    color_attachments: &'a [Option<wgpu::RenderPassColorAttachment<'a>>],
    depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
    timestamp_writes: Option<wgpu::RenderPassTimestampWrites<'a>>,
    occlusion_query_set: Option<&'a wgpu::QuerySet>,
    multiview_mask: Option<NonZero<u32>>,
}

impl<'a> RenderPassBuilder<'a> {
    #[must_use]
    pub fn new(label: &'static str) -> Self {
        Self {
            label: Some(label),
            color_attachments: &[],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        }
    }

    #[must_use]
    pub fn color_attachments(
        mut self,
        value: &'a [Option<wgpu::RenderPassColorAttachment<'a>>],
    ) -> Self {
        self.color_attachments = value;
        self
    }
    #[must_use]
    pub fn depth_stencil_attachment(
        mut self,
        value: wgpu::RenderPassDepthStencilAttachment<'a>,
    ) -> Self {
        self.depth_stencil_attachment = Some(value);
        self
    }
    #[must_use]
    pub fn timestamp_writes(mut self, value: Option<wgpu::RenderPassTimestampWrites<'a>>) -> Self {
        self.timestamp_writes = value;
        self
    }
    #[must_use]
    pub fn occlusion_query_set(mut self, value: Option<&'a wgpu::QuerySet>) -> Self {
        self.occlusion_query_set = value;
        self
    }
    #[must_use]
    pub fn multiview_mask(mut self, value: Option<NonZero<u32>>) -> Self {
        self.multiview_mask = value;
        self
    }

    pub fn load(
        label: &'static str,
        view: &'a wgpu::TextureView,
        encoder: &'a mut wgpu::CommandEncoder,
    ) -> wgpu::RenderPass<'a> {
        let attachments = load_attachments(view);
        Self::new(label)
            .color_attachments(&attachments)
            .begin(encoder)
    }

    pub fn begin(self, encoder: &mut wgpu::CommandEncoder) -> wgpu::RenderPass<'_> {
        encoder.begin_render_pass(&self.into())
    }
}

impl<'a> From<RenderPassBuilder<'a>> for wgpu::RenderPassDescriptor<'a> {
    fn from(value: RenderPassBuilder<'a>) -> Self {
        Self {
            label: value.label,
            color_attachments: value.color_attachments,
            depth_stencil_attachment: value.depth_stencil_attachment,
            timestamp_writes: value.timestamp_writes,
            occlusion_query_set: value.occlusion_query_set,
            multiview_mask: value.multiview_mask,
        }
    }
}

pub fn store<T>(op: wgpu::LoadOp<T>) -> wgpu::Operations<T> {
    wgpu::Operations {
        load: op,
        store: wgpu::StoreOp::Store,
    }
}

#[must_use]
pub fn load_store<T>() -> wgpu::Operations<T> {
    wgpu::Operations {
        load: wgpu::LoadOp::Load,
        store: wgpu::StoreOp::Store,
    }
}

#[must_use]
pub fn op_attachment(
    ops: wgpu::Operations<wgpu::Color>,
    view: &wgpu::TextureView,
) -> wgpu::RenderPassColorAttachment<'_> {
    wgpu::RenderPassColorAttachment {
        view,
        depth_slice: None,
        resolve_target: None,
        ops,
    }
}

#[must_use]
pub fn op_attachments(
    ops: wgpu::Operations<wgpu::Color>,
    view: &wgpu::TextureView,
) -> [Option<wgpu::RenderPassColorAttachment<'_>>; 1] {
    [Some(op_attachment(ops, view))]
}

#[must_use]
pub fn load_attachment(view: &wgpu::TextureView) -> wgpu::RenderPassColorAttachment<'_> {
    op_attachment(load_store(), view)
}

#[must_use]
pub fn load_attachments(
    view: &wgpu::TextureView,
) -> [Option<wgpu::RenderPassColorAttachment<'_>>; 1] {
    [Some(load_attachment(view))]
}
