pub mod handle;

use std::borrow::Cow;

pub trait Material {
    fn shader() -> Shader;
}

pub struct Shader {
    pub source: ShaderSource,
    pub vertex: Option<&'static str>,
    pub fragment: Option<&'static str>,
}

pub enum ShaderSource {
    Single(wgpu::ShaderSource<'static>),
    Split {
        vertex: wgpu::ShaderSource<'static>,
        fragment: wgpu::ShaderSource<'static>,
    },
}

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
    pub fn vertex(&self) -> &wgpu::ShaderModule {
        match self {
            ShaderModules::Single(shader_module) => shader_module,
            ShaderModules::Split {
                vertex,
                fragment: _,
            } => vertex,
        }
    }

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
