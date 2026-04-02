use crate::context::Context;

pub mod slice;
pub mod vec;

pub trait GetBuffer {
    fn get_buffer(&self, ctx: &Context) -> &wgpu::Buffer;
    fn get_length(&self) -> u32;
}
