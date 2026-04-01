use wgpu::include_wgsl;

use crate::prelude::*;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DefaultMaterial {}

impl Material for DefaultMaterial {
    fn shader() -> Shader {
        Shader {
            source: ShaderSource::Single(include_wgsl!("shaders/default.wgsl").source),
            vertex: None,
            fragment: None,
        }
    }
}

impl Default for DefaultMaterial {
    fn default() -> Self {
        Self {}
    }
}
