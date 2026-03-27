use winit::window::{Window, WindowAttributes};

use crate::context::Context;
use crate::graphics::handle::GraphicsConfig;
use crate::logging;
use crate::windowing::event_loop::function_set::FunctionSet;
use crate::windowing::event_loop::{self, EventLoopHandle};

mod builder;

pub struct App<S> {
    init_logger: bool,
    window_attributes: WindowAttributes,
    graphics_config: GraphicsConfig,
    functions: FunctionSet<S>,
}

impl<S: 'static> App<S> {
    fn init(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            console_error_panic_hook::set_once();
        }

        if self.init_logger {
            logging::init();
        }
    }

    pub fn run(mut self) {
        self.init();

        let event_loop = event_loop::create_event_loop().expect("failed to create event loop");
        let event_loop_handle = EventLoopHandle::new(
            self.functions,
            self.window_attributes,
            self.graphics_config,
            &event_loop,
        );

        event_loop_handle.run(event_loop);
    }

    pub fn new(init_state: fn(&mut Context) -> S) -> Self {
        Self {
            init_logger: true,
            window_attributes: Window::default_attributes(),
            graphics_config: GraphicsConfig::default(),
            functions: FunctionSet::<S>::new(init_state),
        }
    }
}
