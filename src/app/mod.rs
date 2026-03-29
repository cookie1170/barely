use std::time::Duration;

use winit::window::{Window, WindowAttributes};

use crate::event_loop::function_set::{FunctionSet, Init};
use crate::event_loop::{self, EventLoopHandle};
use crate::graphics::handle::GraphicsConfig;
use crate::logging;

mod builder;

/// The entrypoint to your app
pub struct App<S> {
    /// Should the logger be initialized (default: true)
    pub init_logger: bool,
    /// The settings for the window (default: [`Window::default_attributes`](winit::window::Window::default_attributes))
    pub window_attributes: WindowAttributes,
    /// Configuration for the underlying graphics api
    pub graphics_config: GraphicsConfig,
    /// How long should a fixed update last
    pub fixed_delta: Duration,
    /// Update and fixed update functions
    pub functions: FunctionSet<S>,
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

    /// Starts the app
    ///
    /// # Panics
    /// When creating or starting the event loop fails
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

        event_loop_handle
            .run(event_loop)
            .expect("failed to start the event loop");
    }

    /// Creates a new [`App`] with a function to initialize the state
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
