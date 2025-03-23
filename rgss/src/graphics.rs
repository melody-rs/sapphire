use std::sync::Arc;

use winit::window::Window as NativeWindow;

mod builders {
    mod binding_helpers;
    pub use binding_helpers::*;
    mod descriptor_helpers;
    pub use descriptor_helpers::*;
    mod shader_helpers;
    pub use shader_helpers::*;
}
use builders::*;

mod bitmap;
pub use bitmap::Bitmap;

mod viewport;
pub use viewport::Viewport;

mod window;
pub use window::Window;

mod sprite;
pub use sprite::Sprite;

mod z;
pub use z::{Drawable, Z, ZList};

use crate::{Arenas, Config, ViewportKey};

pub struct Graphics {
    pub(crate) wgpu: Wgpu,
    pub global_viewport: ViewportKey,
    window: Arc<NativeWindow>,
}

pub(crate) const BITMAP_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

pub(crate) struct Wgpu {
    pub(crate) instance: wgpu::Instance,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,

    pub(crate) features: wgpu::Features,
    pub(crate) limits: wgpu::Limits,
}

#[derive(thiserror::Error, Debug)]
pub enum InitError {
    #[error("Failed to create surface: {0}")]
    SurfaceError(#[from] wgpu::CreateSurfaceError),
    #[error("Failed to find a suitable adapter")]
    NoAdapter,
    #[error("Failed to request a device: {0}")]
    DeviceError(#[from] wgpu::RequestDeviceError),
}

impl Graphics {
    pub async fn new(
        window: Arc<NativeWindow>,
        config: &Config,
        arenas: &mut Arenas,
    ) -> Result<Self, InitError> {
        let wgpu = Wgpu::new(window.clone(), config).await?;
        let viewport = Viewport::global(arenas);
        let global_viewport = arenas.viewports.insert(viewport);
        Ok(Self {
            wgpu,
            window,
            global_viewport,
        })
    }

    pub fn main_window(&self) -> &Arc<NativeWindow> {
        &self.window
    }
}

impl Wgpu {
    pub const fn bindless_features() -> wgpu::Features {
        wgpu::Features::TEXTURE_BINDING_ARRAY
            .union(wgpu::Features::MULTI_DRAW_INDIRECT)
            .union(wgpu::Features::PARTIALLY_BOUND_BINDING_ARRAY)
    }
    pub const fn multi_draw_features() -> wgpu::Features {
        wgpu::Features::MULTI_DRAW_INDIRECT.union(wgpu::Features::INDIRECT_FIRST_INSTANCE)
    }

    pub async fn new(window: Arc<NativeWindow>, config: &Config) -> Result<Self, InitError> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            flags: wgpu::InstanceFlags::from_build_config(),
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;

        let adapter = match wgpu::util::initialize_adapter_from_env(&instance, Some(&surface)) {
            Some(env) => env,
            None => instance
                .request_adapter(&wgpu::RequestAdapterOptionsBase {
                    power_preference: wgpu::PowerPreference::from_env()
                        .unwrap_or(wgpu::PowerPreference::LowPower),
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                })
                .await
                .ok_or(InitError::NoAdapter)?,
        };

        let needed_features = wgpu::Features::PUSH_CONSTANTS;
        let optimization_features = wgpu::Features::CLEAR_TEXTURE
            | wgpu::Features::PIPELINE_CACHE
            | Self::bindless_features()
            | Self::multi_draw_features();
        let requested_features = needed_features | optimization_features;

        let mut features = adapter.features().intersection(requested_features);
        if config.graphics.force_downlevel {
            features = wgpu::Features::default();
        }

        let adapter_limits = adapter.limits();
        let mut limits = wgpu::Limits::default();
        if features.contains(wgpu::Features::PUSH_CONSTANTS) {
            limits.max_push_constant_size = adapter_limits.max_push_constant_size;
        }
        if features.contains(Self::bindless_features()) {
            limits.max_sampled_textures_per_shader_stage =
                adapter_limits.max_sampled_textures_per_shader_stage;
        }

        let surface_config = surface
            .get_default_config(&adapter, 640, 480)
            .expect("surface incompatible with adapter?");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: features,
                    required_limits: limits.clone(),
                    ..Default::default()
                },
                None,
            )
            .await?;

        surface.configure(&device, &surface_config);

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            surface_config,

            features,
            limits,
        })
    }

    pub fn bindless_supported(&self) -> bool {
        self.features.contains(Self::bindless_features())
    }

    pub fn multi_draw_supported(&self) -> bool {
        self.features.contains(Self::multi_draw_features())
    }
}
