use std::cell::Cell;

use crate::context::Context;
use crate::graphics::buffer::GetBuffer;

pub struct VecBuffer<T>
where
    T: bytemuck::Pod,
{
    pub items: Vec<T>,
    pub usage: wgpu::BufferUsages,
    buffer: Cell<wgpu::Buffer>,
}

impl Context<'_> {
    pub fn create_vec_buffer<T>(
        &mut self,
        items: impl Into<Vec<T>>,
        usage: wgpu::BufferUsages,
    ) -> VecBuffer<T>
    where
        T: bytemuck::Pod,
    {
        let usage = usage | wgpu::BufferUsages::COPY_DST;
        let items = items.into();

        let buffer = VecBuffer::<T>::create_buffer(items.len(), usage, &self.device);
        VecBuffer {
            items,
            usage,
            buffer: buffer.into(),
        }
    }
}

impl<T> VecBuffer<T>
where
    T: bytemuck::Pod,
{
    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn usage(&self) -> wgpu::BufferUsages {
        self.usage
    }

    fn create_buffer(len: usize, usage: wgpu::BufferUsages, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vec buffer"),
            size: (len * size_of::<T>()) as u64,
            usage,
            mapped_at_creation: false,
        })
    }
}

impl<T> GetBuffer for VecBuffer<T>
where
    T: bytemuck::Pod,
{
    fn get_buffer(&self, ctx: &Context) -> &wgpu::Buffer {
        let size = (self.items.len() * size_of::<T>()) as u64;
        // SAFETY: if a reference to `self.buffer` exists, then the mutating branch below will not be reached
        let buffer = unsafe { &*self.buffer.as_ptr() };
        if buffer.size() != size {
            let buffer = VecBuffer::<T>::create_buffer(self.items.len(), self.usage, &ctx.device);

            // SAFETY: this branch is only reachable if `self.items` is mutated, which can only happend if no reference to `self.buffer` exists
            unsafe {
                *self.buffer.as_ptr() = buffer;
            }
        }

        // SAFETY: if a reference to `self.buffer` exists, then the mutating branch above will not be reached
        let buffer = unsafe { &*self.buffer.as_ptr() };

        ctx.queue
            .write_buffer(buffer, 0, bytemuck::cast_slice(&self.items));

        buffer
    }

    #[allow(
        clippy::cast_possible_truncation,
        reason = "i don't think anybody is having >2 billion elements in their buffer"
    )]
    fn get_length(&self) -> u32 {
        self.items.len() as u32
    }
}

impl<T: PartialEq> PartialEq for VecBuffer<T>
where
    T: bytemuck::Pod,
{
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items && self.usage == other.usage
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for VecBuffer<T>
where
    T: bytemuck::Pod,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VecBuffer")
            .field("items", &self.items)
            .field("usage", &self.usage)
            .finish_non_exhaustive()
    }
}
