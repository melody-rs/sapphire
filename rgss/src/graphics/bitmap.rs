use wgpu::util::DeviceExt;

use crate::{Arenas, BitmapKey, FileSystem, FontKey, Fonts, Rect};

use super::{Graphics, builders::RenderPassBuilder};

pub struct Bitmap {
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub font: FontKey,
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

#[derive(Default, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum Align {
    #[default]
    Left = 0,
    Center = 1,
    Right = 2,
}

impl Bitmap {
    pub fn new(graphics: &Graphics, font: FontKey, width: u32, height: u32) -> Self {
        // TODO handle bitmaps that are too large
        let texture = graphics
            .wgpu
            .device
            .create_texture(&descriptor_for(width, height));
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            font,
            view,
        }
    }

    pub fn new_path(
        graphics: &Graphics,
        font: FontKey,
        filesystem: &FileSystem,
        path: impl AsRef<camino::Utf8Path>,
    ) -> Self {
        // TODO handle errors
        let mut image_file = filesystem.open_file(path).unwrap();

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

        Self {
            texture,
            font,
            view,
        }
    }

    pub fn width(&self) -> u32 {
        self.texture.width()
    }

    pub fn height(&self) -> u32 {
        self.texture.height()
    }
}

impl BitmapKey {
    pub fn draw_text(
        self,
        arenas: &Arenas,
        fonts: &mut Fonts,
        graphics: &mut Graphics,
        rect: Rect,
        text: impl AsRef<str>,
        align: Align,
    ) {
        let this = &arenas[self];
        let font = &arenas[this.font];
        let color = arenas[font.color];

        let mut text_buffer = glyphon::Buffer::new(
            &mut fonts.font_system,
            glyphon::Metrics::new(font.size as _, font.size as _),
        );
        text_buffer.set_size(
            &mut fonts.font_system,
            Some(rect.width as _),
            Some(rect.height as _),
        );
        text_buffer.set_text(
            &mut fonts.font_system,
            text.as_ref(),
            &glyphon::Attrs::new().family(glyphon::Family::Name(&font.names[0])),
            glyphon::Shaping::Advanced,
        );

        println!("{rect:?}");

        let mut viewport = glyphon::Viewport::new(&graphics.wgpu.device, &fonts.cache);
        viewport.update(
            &graphics.wgpu.queue,
            glyphon::Resolution {
                width: this.width(),
                height: this.height(),
            },
        );

        let text_area = glyphon::TextArea {
            buffer: &text_buffer,
            left: rect.x as _,
            top: rect.y as _,
            scale: 1.0,
            bounds: glyphon::TextBounds {
                left: rect.x,
                top: rect.y,
                right: rect.width as _,
                bottom: rect.height as _,
            },
            default_color: glyphon::Color::rgba(
                color.red as _,
                color.green as _,
                color.blue as _,
                color.alpha as _,
            ),
            custom_glyphs: &[],
        };

        let result = fonts.text_renderer.prepare(
            &graphics.wgpu.device,
            &graphics.wgpu.queue,
            &mut fonts.font_system,
            &mut fonts.atlas,
            &viewport,
            [text_area],
            &mut fonts.swash_cache,
        );
        result.unwrap();

        let rpass_builder =
            RenderPassBuilder::new().with_color_attachment(wgpu::RenderPassColorAttachment {
                view: &this.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            });
        let mut rpass = graphics
            .bitmap_ops
            .begin_render_pass(&rpass_builder.build());

        let result = fonts
            .text_renderer
            .render(&fonts.atlas, &viewport, &mut rpass);
        result.unwrap();

        drop(rpass);
    }

    pub fn text_area(self, arenas: &Arenas, fonts: &mut Fonts, text: impl AsRef<str>) -> Rect {
        let this = &arenas[self];
        let font = &arenas[this.font];

        let mut text_buffer = glyphon::Buffer::new(
            &mut fonts.font_system,
            glyphon::Metrics::new(font.size as _, font.size as _),
        );
        text_buffer.set_text(
            &mut fonts.font_system,
            text.as_ref(),
            &glyphon::Attrs::new().family(glyphon::Family::Name(&font.names[0])),
            glyphon::Shaping::Advanced,
        );

        let mut width = 0.0_f32;
        let mut height = 0.0;
        for run in text_buffer.layout_runs() {
            width = width.max(run.line_w);
            height += run.line_height;
        }

        Rect::new(0, 0, width as _, height as _)
    }
}
