use crate::prelude::*;
use crate::{Context, WeakContext};

type RenderbufferResource = <glow::Context as HasContext>::Renderbuffer;

/// Renderbuffer representation.
pub struct Renderbuffer {
    context  : WeakContext,
    resource : RenderbufferResource
}

impl Renderbuffer {
    /// Creates a default `Renderbuffer`.
    pub fn default(context:&Context) -> Self {
        let resource = Default::default();
        let context = context.weak_ref();
        Self {resource,context}
    }

    /// Creates a new `Renderbuffer` with `(width, height)` dimensions.
    pub fn new(context:&Context, width: u32, height: u32) -> Self {
        let gl       = context.internal_context();
        let width    = width as i32;
        let height   = height as i32;
        let resource = unsafe {
            let resource = gl.create_renderbuffer().expect("Couldn't create Renderbuffer");
            gl.bind_renderbuffer(glow::RENDERBUFFER, Some(resource));
            gl.renderbuffer_storage(glow::RENDERBUFFER, glow::DEPTH_COMPONENT, width, height);
            resource
        };
        let context = context.weak_ref();
        Self {context,resource}
    }

    /// Gets the `RenderbufferResource`.
    pub fn resource(&self) -> RenderbufferResource {
        self.resource
    }
}

impl Drop for Renderbuffer {
    fn drop(&mut self) {
        self.context.upgrade().map(|context| {
            unsafe {
                context.internal_context().delete_renderbuffer(self.resource());
            }
        });
    }
}
