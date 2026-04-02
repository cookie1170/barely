use wgpu::util::DeviceExt;

use crate::context::Context;
use crate::graphics::buffer::GetBuffer;

#[derive(Debug, PartialEq, Clone)]
pub struct SliceBuffer<T>
where
    T: bytemuck::Pod,
{
    items: Box<[T]>,
    usage: wgpu::BufferUsages,
    buffer: wgpu::Buffer,
}

impl Context<'_> {
    pub fn create_slice_buffer<T>(
        &mut self,
        items: impl Into<Box<[T]>>,
        usage: wgpu::BufferUsages,
    ) -> SliceBuffer<T>
    where
        T: bytemuck::Pod,
    {
        let items = items.into();

        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&items),
                usage,
            });

        SliceBuffer {
            items,
            usage,
            buffer,
        }
    }
}

impl<T> SliceBuffer<T>
where
    T: bytemuck::Pod,
{
    #[must_use]
    pub fn items(&self) -> &[T] {
        &self.items
    }

    #[must_use]
    pub fn usage(&self) -> wgpu::BufferUsages {
        self.usage
    }
}

impl<T> GetBuffer for SliceBuffer<T>
where
    T: bytemuck::Pod,
{
    fn get_buffer(&self, _ctx: &Context) -> &wgpu::Buffer {
        &self.buffer
    }

    #[allow(
        clippy::cast_possible_truncation,
        reason = "i don't think anybody is having >2 billion elements in their buffer"
    )]
    fn get_length(&self) -> u32 {
        self.items.len() as u32
    }
}
