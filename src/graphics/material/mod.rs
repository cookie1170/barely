use std::borrow::Cow;

pub mod handle;
pub mod wesl;

pub trait Material: bytemuck::Pod {
    fn shader() -> Shader;

    fn config() -> MeshConfig {
        MeshConfig::default()
    }
}

#[derive(Debug, Clone)]
pub struct Shader {
    pub source: ShaderSource,
    pub vertex: Option<&'static str>,
    pub fragment: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct MeshConfig {
    pub vertex_compilation_options: wgpu::PipelineCompilationOptions<'static>,
    pub fragment_compilation_options: wgpu::PipelineCompilationOptions<'static>,
    pub blend: Option<wgpu::BlendState>,
    pub write_mask: wgpu::ColorWrites,
    pub primitive: wgpu::PrimitiveState,
}

#[derive(Debug, Clone)]
pub enum ShaderSource {
    Single(wgpu::ShaderSource<'static>),
    Split {
        vertex: wgpu::ShaderSource<'static>,
        fragment: wgpu::ShaderSource<'static>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum ShaderModules {
    Single(wgpu::ShaderModule),
    Split {
        vertex: wgpu::ShaderModule,
        fragment: wgpu::ShaderModule,
    },
}

impl Shader {
    pub fn wgsl(source: impl Into<Cow<'static, str>>) -> Self {
        Self {
            source: ShaderSource::Single(wgpu::ShaderSource::Wgsl(source.into())),
            vertex: None,
            fragment: None,
        }
    }
}

impl ShaderModules {
    #[must_use]
    pub fn vertex(&self) -> &wgpu::ShaderModule {
        match self {
            ShaderModules::Single(shader_module) => shader_module,
            ShaderModules::Split {
                vertex,
                fragment: _,
            } => vertex,
        }
    }

    #[must_use]
    pub fn fragment(&self) -> &wgpu::ShaderModule {
        match self {
            ShaderModules::Single(shader_module) => shader_module,
            ShaderModules::Split {
                vertex: _,
                fragment,
            } => fragment,
        }
    }
}

impl ShaderSource {
    #[must_use]
    pub fn create_shader_modules(self, device: &wgpu::Device) -> ShaderModules {
        match self {
            ShaderSource::Single(shader_source) => {
                ShaderModules::Single(device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("single"),
                    source: shader_source,
                }))
            }
            ShaderSource::Split { vertex, fragment } => {
                let vertex = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("vertex"),
                    source: vertex,
                });
                let fragment = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("fragment"),
                    source: fragment,
                });

                ShaderModules::Split { vertex, fragment }
            }
        }
    }
}

impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            vertex_compilation_options: wgpu::PipelineCompilationOptions::default(),
            fragment_compilation_options: wgpu::PipelineCompilationOptions::default(),
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
        }
    }
}
