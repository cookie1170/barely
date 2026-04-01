use std::time::Duration;

use winit::dpi::{LogicalSize, PhysicalSize, Size};
use winit::window::WindowAttributes;

use crate::app::App;
use crate::event_loop::function_set::{FixedUpdate, Update};
use crate::graphics::handle::GraphicsConfig;
use crate::prelude::*;

impl<S> App<S> {
    /// Sets [`init_logger`](App::init_logger)
    pub fn init_logger(&mut self) -> &mut Self {
        self.init_logger = false;
        self
    }

    /// Sets [`fixed_delta`](App::fixed_delta)
    pub fn fixed_delta(&mut self, delta: Duration) -> &mut Self {
        self.fixed_delta = delta;
        self
    }

    /// Sets [`fixed_delta`](App::fixed_delta) using seconds
    pub fn fixed_delta_secs(&mut self, secs: f32) -> &mut Self {
        self.fixed_delta(Duration::from_secs_f32(secs))
    }

    /// Sets the window to be borderless fullscreen
    ///
    /// Doesn't work on WASM
    pub fn borderless_fullscreen(&mut self) -> &mut Self {
        // doesn't work on wasm
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.window_attributes.fullscreen = Some(winit::window::Fullscreen::Borderless(None));
        }

        self
    }

    /// Sets the window title
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.window_attributes.title = title.into();
        self
    }

    /// Sets the window's inner size
    ///
    /// Corresponds to [`WindowAttributes.inner_size`](WindowAttributes::inner_size)
    pub fn inner_size(&mut self, size: Size) -> &mut Self {
        self.window_attributes.inner_size = Some(size);

        self
    }

    /// Sets the window's inner size in physical pixels
    pub fn physical_size(&mut self, size: IVec2) -> &mut Self {
        assert!(size.x > 0 && size.y > 0, "Window size must be positive!");
        let (width, height) = (
            u32::try_from(size.x).unwrap(),
            u32::try_from(size.y).unwrap(),
        );

        self.inner_size(Size::Physical(PhysicalSize::new(width, height)))
    }

    /// Sets the window's inner size in logical pixels
    pub fn logical_size(&mut self, size: Vec2) -> &mut Self {
        assert!(
            size.x > 0.0 && size.y > 0.0,
            "Window size must be positive!"
        );
        let (width, height) = (f64::from(size.x), f64::from(size.y));

        self.inner_size(Size::Logical(LogicalSize::new(width, height)))
    }

    /// Sets the [`window_attributes`](App::window_attributes)
    ///
    /// Note that this overrides settings like borderless fullscreen, title, window size etc. if
    /// this is called after they're set
    pub fn window_attributes(&mut self, attributes: WindowAttributes) -> &mut Self {
        self.window_attributes = attributes;
        self
    }

    /// Sets the [`graphics_config`](App::graphics_config)
    pub fn graphics_config(&mut self, config: GraphicsConfig) -> &mut Self {
        self.graphics_config = config;
        self
    }

    /// Adds an update function which runs every frame
    pub fn update(&mut self, function: Update<S>) -> &mut Self {
        self.functions.update.push(function);
        self
    }

    /// Adds a collection of update functions which run every frame
    pub fn updates<const N: usize>(&mut self, function: [Update<S>; N]) -> &mut Self {
        self.functions.update.extend(function);
        self
    }

    /// Adds a fixed update function, which runs at an interval of [`fixed_delta`](App::fixed_delta)
    pub fn fixed_update(&mut self, function: FixedUpdate<S>) -> &mut Self {
        self.functions.fixed_update.push(function);
        self
    }

    /// Adds a collection of fixed update functions, which run at an interval of [`fixed_delta`](App::fixed_delta)
    pub fn fixed_updates<const N: usize>(&mut self, function: [FixedUpdate<S>; N]) -> &mut Self {
        self.functions.fixed_update.extend(function);
        self
    }
}
