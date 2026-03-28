use std::time::Duration;

use winit::window::WindowAttributes;

use crate::graphics::handle::GraphicsConfig;
use crate::windowing::event_loop::function_set::{FixedUpdate, Update};

impl<S> super::App<S> {
    pub fn init_logger(&mut self) -> &mut Self {
        self.init_logger = false;
        self
    }

    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.window_attributes.title = title.into();
        self
    }

    pub fn fixed_delta(&mut self, delta: Duration) -> &mut Self {
        self.fixed_delta = delta;
        self
    }

    pub fn fixed_delta_secs(&mut self, secs: f32) -> &mut Self {
        self.fixed_delta(Duration::from_secs_f32(secs))
    }

    pub fn borderless_fullscreen(&mut self) -> &mut Self {
        // doesn't work on wasm
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.window_attributes.fullscreen = Some(winit::window::Fullscreen::Borderless(None));
        }

        self
    }

    pub fn window_attributes(&mut self, attributes: WindowAttributes) -> &mut Self {
        self.window_attributes = attributes;
        self
    }

    pub fn graphics_config(&mut self, config: GraphicsConfig) -> &mut Self {
        self.graphics_config = config;
        self
    }

    pub fn update(&mut self, function: Update<S>) -> &mut Self {
        self.functions.update.push(function);
        self
    }

    pub fn updates<const N: usize>(&mut self, function: [Update<S>; N]) -> &mut Self {
        self.functions.update.extend(function);
        self
    }

    pub fn fixed_update(&mut self, function: FixedUpdate<S>) -> &mut Self {
        self.functions.fixed_update.push(function);
        self
    }

    pub fn fixed_updates<const N: usize>(&mut self, function: [FixedUpdate<S>; N]) -> &mut Self {
        self.functions.fixed_update.extend(function);
        self
    }
}
