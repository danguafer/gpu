use crate::prelude::*;
use crate::Context;

use crate::Program;
use crate::FragmentShader;
use crate::VertexShader;
use crate::VertexArrayObject;
use crate::Framebuffer;

/// A program for rasterizing `VertexArrayObject`s in a target `Framebuffer`.
#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct RasterProgram {
    /// Program base object.
    #[shrinkwrap(main_field)]
    pub program : Program,
}

/// Kinds of raster geometries.
pub enum RasterGeometry {
    // /// Raster three consecutive vertices as a triangle.
    // Triangles = gl::TRIANGLES as isize,
    // /// Raster each vertex as a point.
    // Points    = gl::POINTS as isize,
    // /// Raster two consecutive vertices as a line.
    // Lines     = gl::LINES as isize,
    // /// Raster the vertices as a sequence of lines.
    // LineStrip = gl::LINE_STRIP as isize
}

impl RasterProgram {
    /// Creates a new `RasterProgram` with a `FragmentShader` and ` VertexShader`.
    pub fn new(context:&Context, vertex_shader:&VertexShader, fragment_shader:&FragmentShader) -> Result<Self, String> {
        // let program = Program::new(context);
        // unsafe {
        //     gl::AttachShader(program.resource(), vertex_shader.resource());
        //     gl::AttachShader(program.resource(), fragment_shader.resource());
        //     gl::LinkProgram(program.resource());
        //
        //     // Check for linking errors
        //     let mut is_linked = gl::FALSE as i32;
        //     gl::GetProgramiv(program.resource(), gl::LINK_STATUS, &mut is_linked);
        //     if is_linked == gl::FALSE as i32 {
        //         let buffer_size = 4096;
        //         let mut length = 0;
        //         let mut buffer : [u8; 4096] = [0; 4096];
        //         gl::GetProgramInfoLog(program.resource(), buffer_size, &mut length, buffer.as_mut_ptr() as *mut i8);
        //         let err = String::from_raw_parts(buffer.as_mut_ptr(), length as usize, buffer_size as usize);
        //         return Err(err)
        //     }
        // }
        //
        // Ok(Self {program})
        unimplemented!()
    }

    pub(crate) fn use_(&self) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        // }
        unimplemented!()
    }

    /// Draws the `n_vertices` in a `VertexArrayObject` as the specified `RasterGeometry` on the target `Framebuffer`.
    pub fn raster(&self, framebuffer: &Framebuffer, vertex_array_object: &VertexArrayObject, raster_geometry: RasterGeometry, n_vertices: usize) {
        // unsafe {
        //     framebuffer.bind();
        //     self.use_();
        //     vertex_array_object.bind();
        //     gl::Enable(gl::PROGRAM_POINT_SIZE);
        //     let (width,height) = framebuffer.dimensions();
        //     gl::Viewport(0, 0, width as i32, height as i32);
        //     gl::DrawArrays(raster_geometry as u32, 0, n_vertices as i32);
        // }
        unimplemented!()
    }

    /// Raster indexed vertices.
    pub fn indexed_raster(&self, framebuffer: &Framebuffer, vertex_array_object: &VertexArrayObject, raster_geometry: RasterGeometry, n_indices: usize) {
        // unsafe {
        //     framebuffer.bind();
        //     self.use_();
        //     vertex_array_object.bind();
        //     gl::Enable(gl::PROGRAM_POINT_SIZE);
        //     gl::Enable(gl::BLEND);
        //     gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        //     let (width,height) = framebuffer.dimensions();
        //     gl::Viewport(0, 0, width as i32, height as i32);
        //     // gl::DrawArrays(raster_geometry as u32, 0, n_vertices as i32);
        //     // FIXME: Remove hardcoded gl::UNSIGNED_INT. Get the type from vao.index_buffer().type() or something.
        //     gl::DrawElements(raster_geometry as u32, n_indices as i32, gl::UNSIGNED_INT, std::ptr::null());
        // }
        unimplemented!()
    }
}