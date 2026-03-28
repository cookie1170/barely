use std::cell::RefCell;

use crate::context::Context;
use crate::graphics::buffer::GetBuffer;
use crate::graphics::handle::GraphicsHandle;

#[derive(Debug, PartialEq, Clone)]
pub struct VecBuffer<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub items: Vec<T>,
    pub usage: wgpu::BufferUsages,
    buffer: RefCell<wgpu::Buffer>,
}

impl<'a> Context<'a> {
    pub fn create_vec_buffer<T>(
        &mut self,
        items: impl Into<Vec<T>>,
        usage: wgpu::BufferUsages,
    ) -> VecBuffer<T>
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let usage = usage | wgpu::BufferUsages::COPY_DST;
        let items = items.into();

        let buffer = self.handle.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vec buffer"),
            size: (items.len() * size_of::<T>()) as u64,
            usage,
            mapped_at_creation: false,
        });

        VecBuffer {
            items,
            usage,
            buffer: buffer.into(),
        }
    }
}

impl<T> VecBuffer<T>
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

impl<T> GetBuffer for VecBuffer<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    fn get_buffer(&self, handle: &GraphicsHandle) -> &wgpu::Buffer {
        let size = (self.items.len() * size_of::<T>()) as u64;
        if self.buffer.borrow().size() != size {
            let buffer = handle.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Vec buffer"),
                size,
                usage: self.usage(),
                mapped_at_creation: false,
            });

            *self.buffer.borrow_mut() = buffer;
        }
        // SAFETY: `get_buffer` must not be called while a reference to the buffer exists!
        let buffer = unsafe { self.buffer.try_borrow_unguarded() }.unwrap();

        handle
            .queue
            .write_buffer(buffer, 0, bytemuck::cast_slice(&self.items));

        buffer
    }

    fn get_length(&self) -> u32 {
        self.items.len() as u32
    }
}
