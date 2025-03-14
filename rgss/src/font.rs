use crate::{Arenas, Color, ColorKey, Graphics, graphics::BITMAP_TEXTURE_FORMAT};

pub struct Fonts {
    // cosmic text
    font_system: glyphon::FontSystem,
    swash_cache: glyphon::SwashCache,

    cache: glyphon::Cache,
    viewport: glyphon::Viewport,
    atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,

    pub default: Font,
}

// rather than storing colors directly, we store a key to them for garbage collection purposes
pub struct Font {
    pub names: Vec<String>,
    pub size: u32,
    pub bold: bool,
    pub italic: bool,
    pub color: ColorKey,

    pub shadow: bool,
    pub outline: ColorKey,
    pub out_color: ColorKey,
}

impl Fonts {
    pub fn new(graphics: &Graphics, arenas: &mut Arenas) -> Self {
        // cosmic-text's stuff
        let mut font_system = glyphon::FontSystem::new();
        font_system.db_mut().load_fonts_dir("Fonts");

        let swash_cache = glyphon::SwashCache::new();

        // these are pretty expensive to construct, so we only do em once
        let cache = glyphon::Cache::new(&graphics.wgpu.device);
        let viewport = glyphon::Viewport::new(&graphics.wgpu.device, &cache);
        let mut atlas = glyphon::TextAtlas::new(
            &graphics.wgpu.device,
            &graphics.wgpu.queue,
            &cache,
            BITMAP_TEXTURE_FORMAT,
        );
        let text_renderer = glyphon::TextRenderer::new(
            &mut atlas,
            &graphics.wgpu.device,
            wgpu::MultisampleState::default(), // no AA
            None,                              // no depth
        );

        let color = arenas.colors.insert(Color::WHITE);
        let outline = arenas.colors.insert(Color::WHITE);
        let out_color = arenas.colors.insert(Color::GREY);

        let default = Font {
            names: vec!["Arial".to_string()],
            size: 22,
            bold: false,
            italic: false,
            color,

            shadow: false,
            outline,
            out_color,
        };

        Self {
            font_system,
            swash_cache,

            cache,
            viewport,
            atlas,
            text_renderer,

            default,
        }
    }
}
