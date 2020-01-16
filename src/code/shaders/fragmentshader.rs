use crate::code::shaders::shader::create_shader;
use crate::Resource;
use crate::Context;
use glow::HasContext;

pub struct FragmentShader<'context> {
    id      : u32,
    context : &'context Context
}

impl<'context> FragmentShader<'context> {
    pub fn new(context:&'context Context, source:&str) -> Result<Self, String> {
        let id = create_shader(context, glow::FRAGMENT_SHADER, source);
        match id {
            Ok(id) => Ok(Self{ id, context }),
            Err(err) => Err(err)
        }
    }
}

impl<'context> Drop for FragmentShader<'context> {
    fn drop(&mut self) {
        unsafe {
            self.context.gl.delete_shader(self.get_id());
        }
    }
}

impl<'context> Resource for FragmentShader<'context> {
    fn get_id(&self) -> u32 {
        self.id
    }
}
