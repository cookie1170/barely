use std::time::Duration;

use crate::graphics::handle::GraphicsHandle;

pub struct Context<'a> {
    pub handle: &'a mut GraphicsHandle,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub view: &'a wgpu::TextureView,
    pub(crate) delta_time: Duration,
}

pub struct FixedContext {
    pub(crate) delta_time: Duration,
}

impl FixedContext {
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    pub fn delta_secs(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }
}
