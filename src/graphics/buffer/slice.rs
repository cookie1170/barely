use wgpu::util::DeviceExt;

use crate::context::Context;
use crate::graphics::buffer::GetBuffer;
use crate::graphics::handle::GraphicsHandle;

#[derive(Debug, PartialEq, Clone)]
pub struct SliceBuffer<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    items: Box<[T]>,
    usage: wgpu::BufferUsages,
    buffer: wgpu::Buffer,
}

impl<'a> Context<'a> {
    pub fn create_slice_buffer<T>(
        &mut self,
        items: impl Into<Box<[T]>>,
        usage: wgpu::BufferUsages,
    ) -> SliceBuffer<T>
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let items = items.into();

        let buffer = self
            .handle
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
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn usage(&self) -> wgpu::BufferUsages {
        self.usage
    }
}

impl<T> GetBuffer for SliceBuffer<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    fn get_buffer(&self, _handle: &GraphicsHandle) -> &wgpu::Buffer {
        &self.buffer
    }

    fn get_length(&self) -> u32 {
        self.items.len() as u32
    }
}
