use crate::graphics::handle::GraphicsHandle;

pub mod slice;
pub mod vec;

pub trait GetBuffer {
    fn get_buffer(&self, handle: &GraphicsHandle) -> &wgpu::Buffer;
    fn get_length(&self) -> u32;
}
