use crate::prelude::*;

pub struct MaterialHandle<M: Material> {
    inner: M,
    render_pipeline: wgpu::RenderPipeline,
}

impl<M: Material> MaterialHandle<M> {
    pub fn get_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }
}

impl Context<'_> {
    pub fn create_material_handle<M: Material>(&mut self, material: M) -> MaterialHandle<M> {
        let (device, config) = (&self.handle.device, &self.handle.config);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                immediate_size: 0,
            });

        let shader = M::shader();
        let modules = shader.source.create_shader_modules(device);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: modules.vertex(),
                entry_point: shader.vertex,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Vertex::buffer_layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: modules.fragment(),
                entry_point: shader.fragment,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            depth_stencil: None,
            multiview_mask: None,
            cache: None,
        });

        MaterialHandle {
            inner: material,
            render_pipeline,
        }
    }
}

impl<M: Material + PartialEq> PartialEq for MaterialHandle<M> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<M: Material + Clone> Clone for MaterialHandle<M> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            render_pipeline: self.render_pipeline.clone(),
        }
    }
}

impl<M: Material + std::fmt::Debug> std::fmt::Debug for MaterialHandle<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MaterialHandle")
            .field("inner", &self.inner)
            .finish_non_exhaustive()
    }
}
