use std::sync::Arc;

use anyhow::Context;
use winit::dpi::PhysicalSize;
use winit::window::Window;

use crate::prelude::*;

#[derive(Debug)]
pub struct GraphicsHandle {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub window: Arc<Window>,
    pub(crate) camera_buffer: wgpu::Buffer,
    pub(crate) camera_bind_group: wgpu::BindGroup,
    pub(crate) camera_bind_group_layout: wgpu::BindGroupLayout,
}

#[derive(Debug, Clone)]
pub struct GraphicsConfig {
    pub vsync: bool,
    pub force_fallback_adapter: bool,
    pub backends: wgpu::Backends,
    pub power_preference: wgpu::PowerPreference,
    pub backend_options: wgpu::BackendOptions,
    pub memory_budget_thresholds: wgpu::MemoryBudgetThresholds,
    pub memory_hints: wgpu::MemoryHints,
    pub flags: wgpu::InstanceFlags,
    pub required_features: wgpu::Features,
    pub required_limits: wgpu::Limits,
    pub wgpu_experimental_features: wgpu::ExperimentalFeatures,
}

impl GraphicsHandle {
    /// Creates a new [`GraphicsHandle`] with the given [`GraphicsConfig`] and using the given [`Window`]
    ///
    /// # Errors
    /// - when creating a [`wgpu::Surface`] fails
    /// - when requesting a [`wgpu::Adapter`] fails
    /// - when requesting  a [`wgpu::Device`] fails
    pub async fn new(window: Arc<Window>, config: GraphicsConfig) -> anyhow::Result<Self> {
        let mut descriptor = wgpu::InstanceDescriptor {
            display: None,
            backends: config.backends,
            backend_options: config.backend_options,
            memory_budget_thresholds: config.memory_budget_thresholds,
            flags: config.flags,
        };
        descriptor = descriptor.with_env();

        let instance = wgpu::Instance::new(descriptor);
        let surface = instance
            .create_surface(Arc::clone(&window))
            .context("failed to create wgpu surface")?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::from_env()
                    .unwrap_or(config.power_preference),
                force_fallback_adapter: config.force_fallback_adapter,
                compatible_surface: Some(&surface),
            })
            .await
            .context("failed to create wgpu adapter")?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("device"),
                required_features: config.required_features,
                required_limits: config.required_limits,
                experimental_features: config.wgpu_experimental_features,
                memory_hints: config.memory_hints,
                trace: wgpu::Trace::Off,
            })
            .await
            .context("failed to request wgpu device")?;

        let surface_caps = surface.get_capabilities(&adapter);
        // is hardcoding it a good idea? probably not, but who cares anyway
        let surface_format = wgpu::TextureFormat::Rgba8Unorm;

        let config = wgpu::SurfaceConfiguration {
            // RENDER_ATTACHMENT means we render it to the screen
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: if config.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            },
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        let camera_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Camera bind group layout"),
        };

        let camera_bind_group_layout =
            device.create_bind_group_layout(&camera_bind_group_layout_descriptor);

        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera buffer"),
            size: size_of::<Mat4>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group_desciptor = &wgpu::BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        };

        let camera_bind_group = device.create_bind_group(camera_bind_group_desciptor);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            window,
            camera_buffer,
            camera_bind_group,
            camera_bind_group_layout,
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.height == 0 || size.width == 0 {
            return;
        }

        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub(crate) fn get_surface_texture(&self) -> anyhow::Result<GetSurfaceTextureResult> {
        match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => {
                Ok(GetSurfaceTextureResult::Success(surface_texture))
            }
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                self.surface.configure(&self.device, &self.config);
                Ok(GetSurfaceTextureResult::Success(surface_texture))
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                // Skip this frame
                Ok(GetSurfaceTextureResult::Skip)
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                Ok(GetSurfaceTextureResult::Skip)
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                anyhow::bail!("wgpu device lost!");
            }
        }
    }
}

pub(crate) enum GetSurfaceTextureResult {
    Skip,
    Success(wgpu::SurfaceTexture),
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            vsync: true,
            force_fallback_adapter: false,
            power_preference: wgpu::PowerPreference::None,
            backend_options: wgpu::BackendOptions::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            memory_hints: wgpu::MemoryHints::default(),
            flags: wgpu::InstanceFlags::default(),
            required_features: wgpu::Features::empty(),
            wgpu_experimental_features: wgpu::ExperimentalFeatures::disabled(),
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            #[cfg(not(target_arch = "wasm32"))]
            required_limits: wgpu::Limits::defaults(),
            #[cfg(target_arch = "wasm32")]
            required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
        }
    }
}
