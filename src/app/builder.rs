use winit::window::WindowAttributes;

use crate::context::Context;
use crate::graphics::handle::GraphicsConfig;

impl<S: Default> super::App<S> {
    pub fn init_logger(&mut self) -> &mut Self {
        self.init_logger = false;
        self
    }

    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.window_attributes.title = title.into();
        self
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

    pub fn update(&mut self, function: fn(&mut S, &mut Context)) -> &mut Self {
        self.functions.update.push(function);
        self
    }

    pub fn updates<const N: usize>(
        &mut self,
        function: [fn(&mut S, &mut Context); N],
    ) -> &mut Self {
        self.functions.update.extend(function);
        self
    }

    pub fn fixed_update(&mut self, function: fn(&mut S, &mut Context)) -> &mut Self {
        self.functions.fixed_update.push(function);
        self
    }

    pub fn fixed_updates<const N: usize>(
        &mut self,
        function: [fn(&mut S, &mut Context); N],
    ) -> &mut Self {
        self.functions.fixed_update.extend(function);
        self
    }
}
