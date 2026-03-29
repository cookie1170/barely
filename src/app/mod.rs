use std::time::Duration;

use winit::window::{Window, WindowAttributes};

use crate::event_loop::function_set::{FunctionSet, Init};
use crate::event_loop::{self, EventLoopHandle};
use crate::graphics::handle::GraphicsConfig;
use crate::logging;

mod builder;

pub struct App<S> {
    init_logger: bool,
    window_attributes: WindowAttributes,
    graphics_config: GraphicsConfig,
    fixed_delta: Duration,
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
            self.fixed_delta,
            &event_loop,
        );

        event_loop_handle.run(event_loop);
    }

    pub fn new(init_state: Init<S>) -> Self {
        Self {
            init_logger: true,
            fixed_delta: Duration::from_secs_f32(1.0 / 60.0),
            window_attributes: Window::default_attributes(),
            graphics_config: GraphicsConfig::default(),
            functions: FunctionSet::<S>::new(init_state),
        }
    }
}

impl<S: Default + 'static> Default for App<S> {
    fn default() -> Self {
        App::new(|_| S::default())
    }
}
