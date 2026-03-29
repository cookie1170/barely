use std::sync::Arc;
use std::time::Duration;

use instant::Instant;
use log::error;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::PhysicalKey;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;
use winit::window::WindowAttributes;

use crate::context::{Context, FixedContext};
use crate::event_loop::input::InputState;
use crate::graphics::handle::{GetSurfaceTextureResult, GraphicsConfig, GraphicsHandle};

mod context;
pub mod function_set;
pub mod input;

/// A handle to the [`EventLoop`](`winit::event_loop::EventLoop`),
/// which includes relevant info
pub struct EventLoopHandle<S: 'static> {
    /// An [`EventLoopProxy`](winit::event_loop::EventLoopProxy), which is required on WASM because the creation of a [`GraphicsHandle`] is async
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<GraphicsHandle>>,
    /// A handle to the underlying graphics API (wgpu)
    ///
    /// See [`GraphicsHandle`]
    handle: Option<GraphicsHandle>,
    /// Update and fixed update functions
    functions: function_set::FunctionSet<S>,
    /// Settings for the window
    window_attributes: WindowAttributes,
    /// Configuration for the graphics API
    graphics_config: GraphicsConfig,
    /// When the last frame started (used for delta time calculation)
    last_frame: Instant,
    /// How long behind are we on fixed updates? A fixed update is triggered when this exceeds [`fixed_delta`]
    fixed_buildup: Duration,
    /// How much time should pass between fixed updates
    fixed_delta: Duration,
    /// The current input state
    input_state: InputState,
    /// The current input state for fixed updates. Has to be different because otherwise `just_pressed` events may get swallowed
    fixed_input_state: InputState,
    /// The current state of the app
    state: Option<S>,
}

impl<S> ApplicationHandler<GraphicsHandle> for EventLoopHandle<S> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = self.window_attributes.clone();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "canvas";

            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let graphics_config = self.graphics_config.clone();

        #[cfg(not(target_arch = "wasm32"))]
        {
            // if we are not on web we can use pollster
            self.handle =
                Some(pollster::block_on(GraphicsHandle::new(window, graphics_config)).unwrap());
        }

        #[cfg(target_arch = "wasm32")]
        {
            // on web we have to run the future asynchronously and use the
            // proxy to send the results to the event loop
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    let handle = GraphicsHandle::new(window, graphics_config)
                        .await
                        .expect("Unable to create canvas!!!");

                    assert!(proxy.send_event(handle).is_ok());
                });
            }
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: GraphicsHandle) {
        // ihis is where proxy.send_event() ends up
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            event.resize(event.window.inner_size());
        }

        self.handle = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(handle) = &mut self.handle else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => handle.resize(size),
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let delta_time = now - self.last_frame;
                self.last_frame = now;
                self.fixed_buildup += delta_time;

                while self.fixed_buildup > self.fixed_delta
                    && let Some(state) = &mut self.state
                {
                    self.fixed_buildup -= self.fixed_delta;
                    let fixed_context = FixedContext {
                        delta_time: self.fixed_delta,
                        input_state: &self.fixed_input_state,
                    };

                    self.functions.run_fixed_update(state, &fixed_context);

                    // we ran the update, now clear out the input
                    self.fixed_input_state.on_update();
                }

                handle.window.request_redraw();

                if handle.surface.get_configuration().is_none() {
                    log::warn!("not configured");
                    // not configured, can't use it!
                    return;
                }

                let output = match handle.get_surface_texture() {
                    Ok(r) => match r {
                        GetSurfaceTextureResult::Skip => return,
                        GetSurfaceTextureResult::Success(surface_texture) => surface_texture,
                    },
                    Err(err) => {
                        error!("{err}");
                        return;
                    }
                };

                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder =
                    handle
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Command encoder"),
                        });

                // handle
                //     .window
                //     .set_title(&format!("FPS: {:.02}", 1.0 / delta_time.as_secs_f32()));

                let mut context = Context {
                    handle,
                    view: &view,
                    encoder: &mut encoder,
                    delta_time,
                    input_state: &self.input_state,
                };

                let state = self
                    .state
                    .get_or_insert_with(|| self.functions.get_state(&mut context));

                self.functions.run_update(state, &mut context);
                // we ran the update, now clear the input
                self.input_state.on_update();

                handle.queue.submit(std::iter::once(encoder.finish()));
                output.present();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key,
                        state,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                let PhysicalKey::Code(key) = physical_key else {
                    return;
                };

                match state {
                    winit::event::ElementState::Pressed => {
                        self.input_state.on_pressed(key);
                        self.fixed_input_state.on_pressed(key);
                    }
                    winit::event::ElementState::Released => {
                        self.input_state.on_released(key);
                        self.fixed_input_state.on_released(key);
                    }
                }
            }
            WindowEvent::Focused(false) => {
                self.input_state.on_focus_lost();
                self.fixed_input_state.on_focus_lost();
            }
            _ => (),
        }
    }
}

impl<S> EventLoopHandle<S> {
    /// Creates a new [`EventLoopHandle`] using the given parameters
    pub fn new(
        functions: function_set::FunctionSet<S>,
        window_attributes: WindowAttributes,
        graphics_config: GraphicsConfig,
        fixed_delta: Duration,
        #[allow(unused, reason = "used on wasm")] event_loop: &EventLoop<GraphicsHandle>,
    ) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        Self {
            #[cfg(target_arch = "wasm32")]
            proxy,
            functions,
            window_attributes,
            graphics_config,
            last_frame: Instant::now(),
            // initialize it to fixed_delta to have a fixed update right away
            fixed_buildup: fixed_delta,
            fixed_delta,
            state: None,
            handle: None,
            input_state: InputState::default(),
            fixed_input_state: InputState::default(),
        }
    }

    /// Runs the event loop
    ///
    /// # Errors
    /// When `EventLoop::run_app` fails (see [`EventLoopError`](winit::error::EventLoopError))
    pub fn run(
        #[allow(unused_mut, reason = "used on native but not wasm")] mut self,
        event_loop: EventLoop<GraphicsHandle>,
    ) -> anyhow::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            event_loop.run_app(&mut self)?;
        }
        #[cfg(target_arch = "wasm32")]
        {
            event_loop.spawn_app(self);
        }

        Ok(())
    }
}

/// Creates a new [`EventLoop`](winit::event_loop::EventLoop)
///
/// # Errors
/// When creating the event loop fails (see [`EventLoopError`](winit::error::EventLoopError))
pub fn create_event_loop() -> Result<EventLoop<GraphicsHandle>, EventLoopError> {
    EventLoop::with_user_event().build()
}
