use crate::context::ContextBuilder;
use crate::context::ContextDisplay;
use crate::Framebuffer;

pub use glutin::ContextError;
use glutin::ContextTrait;



// ===============
// === Context ===
// ===============

pub struct Context {
    events_loop : glutin::EventsLoop,
    context     : glutin::WindowedContext,
    pub gl      : glow::Context
}

impl Context {
    pub fn new(builder:&ContextBuilder) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let mut window_builder = glutin::WindowBuilder::new();

        match &builder.display {
            ContextDisplay::Window(name, width, height) => {
                window_builder = window_builder.with_title(name)
                    .with_dimensions(glutin::dpi::LogicalSize::new(*width as f64, *height as f64));
            },
            ContextDisplay::Screen => {
                window_builder = window_builder.with_title("")
                    .with_fullscreen(Some(events_loop.get_primary_monitor()));
            },
            ContextDisplay::None => {
                window_builder = window_builder.with_title("")
                    .with_fullscreen(Some(events_loop.get_primary_monitor()))
                    .with_visibility(false);
            }
        }

        let context = match builder.display {
            ContextDisplay::Window(_, _, _) | ContextDisplay::Screen => {
                glutin::ContextBuilder::new().with_vsync(builder.vsync)
                    .build_windowed(window_builder, &events_loop)
                    .unwrap()
            },
            ContextDisplay::None => {
                glutin::ContextBuilder::new().with_vsync(builder.vsync)
                    .build_windowed(window_builder, &events_loop) // the guideline for creating a headless context is: try build_headless, if it fails, fallback to hidden window
                    .unwrap()
            }
        };

        context.hide_cursor(!builder.cursor);

        let gl = glow::Context::from_loader_function(|s| {
            context.get_proc_address(s) as *const _
        });

        Self{events_loop,context,gl}
    }


    pub fn run(&mut self) -> bool {
        let events_loop = &mut self.events_loop;
        let context = &mut self.context;
        let mut available = true;
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent{ event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => available = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = context.get_hidpi_factor();
                        context.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => ()
                }
            }
        });
        available
    }

    pub fn make_current(&self) -> Result<(), ContextError> {
        unsafe {
            self.context.make_current()

        }
    }

    pub fn swap_buffers(&self) -> Result<(), ContextError> {
        self.context.swap_buffers()
    }

    pub fn get_proc_address(&self, addr: &str) -> *const () {
        self.context.get_proc_address(addr)
    }

    pub fn inner_dimensions(&self) -> (usize, usize) {
        let dpi      = self.context.get_hidpi_factor();
        let logical  = self.context.get_inner_size().expect("Couldn't get inner size");
        let physical = logical.to_physical(dpi);
        (physical.width as usize, physical.height as usize)
    }
}
