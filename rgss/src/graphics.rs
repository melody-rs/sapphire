use std::{sync::Arc, time::Duration, time::Instant};

use wgpu::PipelineCache;
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
pub use bitmap::{Align, Bitmap};

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
    pub(crate) pipeline_cache: Option<PipelineCache>,
    bitmap_ops: wgpu::CommandEncoder,

    last_frame: Instant,
    pub frame_rate: u16,
    pub frame_count: u64,

    #[cfg(debug_assertions)]
    pub(crate) capture_frame: bool,

    pub global_viewport: ViewportKey,

    window: Arc<NativeWindow>,
}

pub(crate) const BITMAP_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
pub(crate) const BITMAP_OPS_DESC: wgpu::CommandEncoderDescriptor = wgpu::CommandEncoderDescriptor {
    label: Some("sapphire bitmap operations"),
};

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
    NoAdapter(#[from] wgpu::RequestAdapterError),
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

        let adapter_info = wgpu.adapter.get_info();
        log::info!(
            "Selected adapter {} {}",
            adapter_info.name,
            adapter_info.driver
        );
        let (pipeline_cache, key) = wgpu::util::pipeline_cache_key(&adapter_info)
            .filter(|_| wgpu.cache_supported())
            .map(|key| {
                log::info!("Loading pipeline cache from {key}");
                let data = std::fs::read(&key)
                    .inspect_err(|err| log::error!("Failed to read pipeline cache: {err}!"))
                    .ok();
                let desc = wgpu::PipelineCacheDescriptor {
                    label: Some(&key),
                    data: data.as_deref(),
                    fallback: true,
                };
                let cache = unsafe { wgpu.device.create_pipeline_cache(&desc) };
                (cache, key)
            })
            .unzip();

        // TODO shaders

        if let Some(data) = pipeline_cache
            .as_ref()
            .and_then(wgpu::PipelineCache::get_data)
        {
            let key = key.unwrap(); // should be Some() if cache is Some()
            if let Err(err) = std::fs::write(key, data) {
                log::error!("Failed to write pipeline cache: {err}!");
            }
        }

        let bitmap_ops = wgpu.device.create_command_encoder(&BITMAP_OPS_DESC);

        let viewport = Viewport::global(arenas);
        let global_viewport = arenas.viewports.insert(viewport);

        let this = Self {
            wgpu,
            pipeline_cache,

            bitmap_ops,

            last_frame: Instant::now(),
            frame_rate: 40,
            frame_count: 0,

            capture_frame: false,

            global_viewport,

            window,
        };

        Ok(this)
    }

    pub fn main_window(&self) -> &Arc<NativeWindow> {
        &self.window
    }

    pub fn update(&mut self) {
        let result = self.wgpu.surface.get_current_texture();
        let surface_texture = match result {
            Ok(t) => t,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                log::warn!("Failed to get current surface texture, reaquiring");
                self.wgpu
                    .surface
                    .configure(&self.wgpu.device, &self.wgpu.surface_config);
                return;
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                log::error!("Out of memory!");
                std::process::abort();
            }
            Err(e) => {
                log::error!("Failed to get current surface texture: {e}");
                return;
            }
        };
        let texture_view = surface_texture.texture.create_view(&Default::default());

        if self.capture_frame {
            unsafe { self.wgpu.device.start_graphics_debugger_capture() }
        }

        let new_bitmap_ops = self.wgpu.device.create_command_encoder(&BITMAP_OPS_DESC);
        let bitmap_ops = std::mem::replace(&mut self.bitmap_ops, new_bitmap_ops);
        let bitmap_ops_buf = bitmap_ops.finish();

        let mut render_encoder = self.wgpu.device.create_command_encoder(&Default::default());

        let rpass_builder =
            RenderPassBuilder::new().with_color_attachment(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            });
        let rpass_desc = rpass_builder.build();

        let rpass = render_encoder.begin_render_pass(&rpass_desc);
        drop(rpass); // drop to finish it

        let render_encoder_buf = render_encoder.finish();
        self.wgpu.queue.submit([bitmap_ops_buf, render_encoder_buf]);

        self.window.pre_present_notify();
        surface_texture.present();

        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame);

        // if the current time took less time to render than the the total frame time,
        // then sleep for the leftover
        let full_frame_time = Duration::from_secs_f32(1.0 / self.frame_rate as f32);
        if let Some(sleep_time) = full_frame_time.checked_sub(frame_time) {
            std::thread::sleep(sleep_time);
        }
        self.last_frame = Instant::now();
        self.frame_count += 1;

        if self.capture_frame {
            unsafe { self.wgpu.device.stop_graphics_debugger_capture() };
        }
        self.capture_frame = false;
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
            Ok(env) => env,
            Err(e) => {
                log::error!("Failed to find adapter: {e}");
                instance
                    .request_adapter(&wgpu::RequestAdapterOptionsBase {
                        power_preference: wgpu::PowerPreference::from_env()
                            .unwrap_or(wgpu::PowerPreference::LowPower),
                        force_fallback_adapter: false,
                        compatible_surface: Some(&surface),
                    })
                    .await?
            }
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
            .request_device(&wgpu::DeviceDescriptor {
                required_features: features,
                required_limits: limits.clone(),
                ..Default::default()
            })
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

    pub fn cache_supported(&self) -> bool {
        self.features.contains(wgpu::Features::PIPELINE_CACHE)
    }
}
