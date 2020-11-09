use crate::prelude::*;
use crate::Context;

use crate::data::as_u8_slice;

use crate::TextureFormat;
use crate::ColorFormat;
use crate::Type;
use crate::Texture;
use std::ffi::c_void;

/// A `Texture3D` representation.
#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Texture3D {
    /// Base texture object.
    #[shrinkwrap(main_field)]
    pub texture : Texture,
    dimensions  : (usize,usize,usize)
}

impl Texture3D {
    fn new(context:&Context) -> Self {
        let format     = TextureFormat::new(ColorFormat::RGBA, Type::F32);
        let texture    = Texture::new(context,format,gl::TEXTURE_3D);
        let dimensions = (0,0,0);
        Self {texture,dimensions}
    }

    /// Gets the dimensions.
    pub fn dimensions(&self) -> (usize, usize, usize) {
        self.dimensions
    }

    /// Allocates a new `Texture3D` with the specified dimensions and `TextureFormat`.
    pub fn allocate
    (context:&Context, dimensions: (usize, usize, usize), format: &TextureFormat) -> Self {
        let mut texture = Self::new(context);
        texture.reallocate(dimensions, &format);
        texture
    }

    /// Creates a new `Texture3D` from a slice.
    pub fn from_data<T>
    (context:&Context, dimensions: (usize, usize, usize), format: &TextureFormat, data: &[T], data_format: &TextureFormat) -> Self {
        let mut texture = Self::new(context);
        texture.set_data(dimensions, &format, data, &data_format);
        texture
    }

    /// Reallocates the memory on the GPU side.
    pub fn reallocate(&mut self, dimensions: (usize, usize, usize), format: &TextureFormat) {
        self.dimensions = dimensions;
        self.format = format.clone();
        self.bind();
        unsafe {
            let tex_type        = self.type_();
            let internal_format = format.internal_format();
            gl::TexStorage3D(tex_type, 1, internal_format, dimensions.0 as i32, dimensions.1 as
                i32, dimensions.2 as i32);
        }
    }

    /// Sets the data on the GPU side.
    pub fn set_data<T>(&mut self, dimensions: (usize, usize, usize), format: &TextureFormat,
                       data: &[T], data_format: &TextureFormat) {
        self.dimensions = dimensions;
        self.format = format.clone();
        self.bind();
        unsafe {
            let (color, ty)     = data_format.get_format_type();
            let internal_format = format.internal_format() as i32;
            let width           = dimensions.0 as i32;
            let height          = dimensions.1 as i32;
            let depth           = dimensions.2 as i32;
            let pixels          = as_u8_slice(data);
            gl::TexImage3D(self.type_(),0,internal_format,width,height,depth,0,color,ty,data.as_ptr() as *const c_void);
        }
    }

    /// Gets a copy of the data on the GPU.
    pub fn data<T>(&self) -> Vec<T> {
        let (width,height,depth) = self.dimensions();
        let color_size           = self.format().color_format().size();
        let capacity             = width * height * depth * color_size;
        let mut data : Vec<T>    = Vec::with_capacity(capacity);
        unsafe {
            data.set_len(capacity);
            let (format, type_) = self.texture.format().get_format_type();

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_3D, self.resource());
            gl::GetTexImage(gl::TEXTURE_3D, 0, format, type_, data.as_mut_ptr() as *mut std::ffi::c_void);
        }
        data
    }
}