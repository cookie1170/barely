use std::time::Duration;

use winit::window::Window;

use crate::prelude::*;

pub struct Context<'a> {
    pub surface: &'a mut wgpu::Surface<'static>,
    pub device: &'a mut wgpu::Device,
    pub queue: &'a mut wgpu::Queue,
    pub config: &'a mut wgpu::SurfaceConfiguration,
    pub window: &'a Window,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub view: &'a wgpu::TextureView,
    pub(crate) delta_time: Duration,
    pub(crate) input_state: &'a InputState,
    pub(crate) events: &'a [winit::event::WindowEvent],
    pub(crate) should_exit: bool,
    pub(crate) camera_buffer: &'a wgpu::Buffer,
    pub(crate) camera_bind_group: &'a wgpu::BindGroup,
    pub(crate) camera_bind_group_layout: &'a wgpu::BindGroupLayout,
}

pub struct FixedContext<'a> {
    pub(crate) delta_time: Duration,
    pub(crate) input_state: &'a InputState,
}

impl FixedContext<'_> {
    #[must_use]
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    #[must_use]
    pub fn delta_secs(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }
}
