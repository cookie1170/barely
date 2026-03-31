use crate::prelude::*;

pub struct MaterialHandle<M: Material> {
    pub inner: M,
    render_pipeline: wgpu::RenderPipeline,
    bind_group: Option<wgpu::BindGroup>,
    uniform_buffer: Option<wgpu::Buffer>,
}

impl<M: Material> MaterialHandle<M> {
    pub fn get_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }

    pub fn update_pass(&self, pass: &mut wgpu::RenderPass) {
        let Some(bind_group) = &self.bind_group else {
            return;
        };

        pass.set_bind_group(1, Some(bind_group), &[]);
    }

    fn create_uniform_buffer(device: &wgpu::Device) -> Option<wgpu::Buffer> {
        if size_of::<M>() == 0 {
            return None;
        }
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform buffer"),
            size: size_of::<M>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Some(buffer)
    }

    fn create_bind_group(
        device: &wgpu::Device,
        uniform_buffer: &wgpu::Buffer,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            label: Some("Custom material bind group"),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: uniform_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        })
    }

    fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Custom material bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }
}

impl Context<'_> {
    pub fn create_material_handle<M: Material>(&mut self, material: M) -> MaterialHandle<M> {
        let (device, config) = (&self.handle.device, &self.handle.config);

        let (bind_group_layout, bind_group, uniform_buffer) = 'create_group: {
            let uniform_buffer = MaterialHandle::<M>::create_uniform_buffer(&self.handle.device);

            let Some(uniform_buffer) = uniform_buffer else {
                break 'create_group (None, None, None);
            };

            let bind_group_layout = MaterialHandle::<M>::create_bind_group_layout(device);

            let bind_group =
                MaterialHandle::<M>::create_bind_group(device, &uniform_buffer, &bind_group_layout);

            (
                Some(bind_group_layout),
                Some(bind_group),
                Some(uniform_buffer),
            )
        };

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[None, bind_group_layout.as_ref()],
                immediate_size: 0,
            });

        let shader = M::shader();
        let modules = shader.source.create_shader_modules(device);
        let mesh_config = M::config();

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: modules.vertex(),
                entry_point: shader.vertex,
                compilation_options: mesh_config.vertex_compilation_options,
                buffers: &[Vertex::buffer_layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: modules.fragment(),
                entry_point: shader.fragment,
                compilation_options: mesh_config.fragment_compilation_options,
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: mesh_config.blend,
                    write_mask: mesh_config.write_mask,
                })],
            }),
            primitive: mesh_config.primitive,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            depth_stencil: None,
            multiview_mask: None,
            cache: None,
        });

        let mut handle = MaterialHandle {
            inner: material,
            render_pipeline,
            bind_group,
            uniform_buffer,
        };

        self.update_material_handle(&mut handle);

        handle
    }

    pub fn update_material_handle<M: Material>(&mut self, handle: &mut MaterialHandle<M>) {
        let Some(uniform_buffer) = &handle.uniform_buffer else {
            return;
        };

        self.handle
            .queue
            .write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[handle.inner]));
    }

    pub fn update_material_handle_size<M: Material>(&mut self, handle: &mut MaterialHandle<M>) {
        handle.uniform_buffer = MaterialHandle::<M>::create_uniform_buffer(&self.handle.device);

        let Some(uniform_buffer) = &handle.uniform_buffer else {
            return;
        };

        let layout = MaterialHandle::<M>::create_bind_group_layout(&self.handle.device);
        handle.bind_group = Some(MaterialHandle::<M>::create_bind_group(
            &self.handle.device,
            uniform_buffer,
            &layout,
        ));
        self.update_material_handle(handle);
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
            inner: self.inner,
            render_pipeline: self.render_pipeline.clone(),
            uniform_buffer: self.uniform_buffer.clone(),
            bind_group: self.bind_group.clone(),
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
