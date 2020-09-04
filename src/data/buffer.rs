use crate::Context;

use glow::HasContext;

type BufferResource = <glow::Context as HasContext>::Buffer;

use super::as_u8_mut_slice;
use super::as_u8_slice;

/// A `Buffer` representation.
pub struct Buffer<'context> {
    context  : &'context Context,
    resource : BufferResource
}

impl<'context> Buffer<'context> {
    fn new(context:&'context Context) -> Buffer<'context> {
        let resource = unsafe {
            context.gl.create_buffer().expect("Couldn't create Buffer")
        };
        Buffer {context,resource}
    }

    /// Gets the `BufferResource`.
    pub fn resource(&self) -> BufferResource {
        self.resource
    }

    /// Creates a new `Buffer` from a slice.
    pub fn from_data<T>(context:&'context Context, data: &[T]) -> Buffer<'context> {
        let mut buffer = Buffer::new(context);
        buffer.set_data(data);
        buffer
    }

    /// Allocates a new `Buffer` with `n_bytes`.
    pub fn allocate(context:&'context Context, n_bytes:usize) -> Buffer<'context> {
        let mut buffer = Buffer::new(context);
        if n_bytes > 0 { buffer.reallocate(n_bytes); }
        buffer
    }

    pub(crate) fn bind(&self) {
        let gl = &self.context.gl;
        let resource = self.resource();
        let resource = if resource == Default::default() { None } else { Some(resource) };
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, resource);
        }
    }

    /// Gets the size in bytes.
    pub fn size(&self) -> usize {
        let gl = &self.context.gl;
        self.bind();
        unsafe {
            gl.get_buffer_parameter_i32(glow::ARRAY_BUFFER, glow::BUFFER_SIZE) as usize
        }
    }

    /// Sets the data on the GPU side.
    pub fn set_data<T>(&mut self, data: &[T]) {
        let gl = &self.context.gl;
        self.bind();
        unsafe {
            let slice = as_u8_slice(data.as_ref());
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, slice, glow::STATIC_DRAW);
        }
    }

    /// Gets the data on the GPU side.
    pub fn data<T>(&self) -> Vec<T> {
        let gl = &self.context.gl;
        self.bind();

        let size = self.size();
        let capacity = size / std::mem::size_of::<T>();
        let mut data : Vec<T> = Vec::with_capacity(capacity);
        unsafe {
            data.set_len(capacity);
            let offset = 0;
            let data   = as_u8_mut_slice(data.as_mut());
            gl.get_buffer_sub_data(glow::ARRAY_BUFFER, offset, data);
        }
        data
    }

    /// Reallocates the memory with `size`.
    pub fn reallocate(&mut self, size: usize) {
        let gl = &self.context.gl;
        self.bind();
        unsafe {
            gl.buffer_data_size(glow::ARRAY_BUFFER, size as i32, glow::STATIC_DRAW);
        }
    }
}

impl Drop for Buffer<'_> {
    fn drop(&mut self) {
        unsafe {
            self.context.gl.delete_buffer(self.resource());
        }
    }
}