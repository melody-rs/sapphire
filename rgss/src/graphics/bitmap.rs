use wgpu::util::DeviceExt;

use crate::FileSystem;

use super::Graphics;

pub struct Bitmap {
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
}

fn descriptor_for(width: u32, height: u32) -> wgpu::TextureDescriptor<'static> {
    wgpu::TextureDescriptor {
        label: Some("sapphire bitmap"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    }
}

impl Bitmap {
    pub fn new(graphics: &Graphics, width: u32, height: u32) -> Self {
        // TODO handle bitmaps that are too large
        let texture = graphics
            .wgpu
            .device
            .create_texture(&descriptor_for(width, height));
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { texture, view }
    }

    pub fn new_path(
        graphics: &Graphics,
        filesystem: &FileSystem,
        path: impl AsRef<camino::Utf8Path>,
    ) -> Self {
        // TODO handle errors
        let mut image_file = filesystem.read_file(path).unwrap();

        let mut image_data = vec![];
        image_file.read_to_end(&mut image_data).unwrap();

        let image = image::load_from_memory(&image_data).unwrap().to_rgba8();

        let texture = graphics.wgpu.device.create_texture_with_data(
            &graphics.wgpu.queue,
            &descriptor_for(image.width() as u32, image.height() as u32),
            wgpu::util::TextureDataOrder::LayerMajor,
            &image,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { texture, view }
    }

    pub fn width(&self) -> u32 {
        self.texture.width()
    }

    pub fn height(&self) -> u32 {
        self.texture.height()
    }
}
