use crate::prelude::*;
use crate::data::Buffer;
use crate::{Context, GLContext};

type VertexArrayObjectResource = <glow::Context as HasContext>::VertexArray;

/// `VertexArrayObject` representation.
pub struct VertexArrayObject {
    gl       : GLContext,
    resource : VertexArrayObjectResource,
    vertices : u32
}

impl VertexArrayObject {
    /// Creates a new `VertexArrayObject`.
    pub fn new(context:&Context) -> Self {
        let gl = context.gl_context();
        let resource = unsafe {
            gl.create_vertex_array().expect("Couldn't create VertexArrayObject")
        };
        let vertices = 0;
        Self { gl, resource, vertices }
    }

    pub(crate) fn resource(&self) -> VertexArrayObjectResource {
        self.resource
    }

    pub(crate) fn bind(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.resource()));
        }
    }

    /// Sets a `Buffer` as a vertices sources, where each vertex has `n_elements`
    pub fn set_vertex_buffer(&mut self, buffer : &Buffer, attribute_index: u32, n_elements: u32) {
        let gl = &self.gl;
        self.bind();
        buffer.bind();
        unsafe {
            gl.enable_vertex_attrib_array(attribute_index);
            gl.vertex_attrib_pointer_f32(attribute_index, n_elements as i32, glow::FLOAT, false, 0, 0);
        }
    }

    /// Sets the number of vertices.
    pub fn set_vertices(&mut self, vertices : u32) {
        self.vertices = vertices;
    }

    /// Gets the number of vertices.
    pub fn get_vertices(&self) -> u32 {
        self.vertices
    }

    // TODO:
    // pub fn set_index_buffer(&mut self, buffer : &Buffer, elements: u32) {
    //     unsafe {
    //         gl::BindVertexArray(self.id);
    //         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.id);
    //         gl::
    //     }
    // }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.resource());
        }
    }
}
