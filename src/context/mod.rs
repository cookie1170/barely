use std::time::Duration;

use crate::graphics::handle::GraphicsHandle;
use crate::prelude::*;

pub struct Context<'a> {
    pub handle: &'a mut GraphicsHandle,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub view: &'a wgpu::TextureView,
    pub(crate) delta_time: Duration,
    pub(crate) input_state: &'a InputState,
    pub(crate) events: &'a [winit::event::WindowEvent],
    pub(crate) should_exit: bool,
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
