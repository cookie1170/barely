use crate::graphics::handle::GraphicsHandle;

mod graphics;

pub struct Context<'a> {
    pub handle: &'a mut GraphicsHandle,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub view: &'a wgpu::TextureView,
}
