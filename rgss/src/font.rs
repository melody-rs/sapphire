use crate::{Arenas, Color, ColorKey, FontKey, Graphics, graphics::BITMAP_TEXTURE_FORMAT};

pub struct Fonts {
    // cosmic text
    pub(crate) font_system: glyphon::FontSystem,
    pub(crate) swash_cache: glyphon::SwashCache,

    pub(crate) cache: glyphon::Cache,
    pub(crate) atlas: glyphon::TextAtlas,
    pub(crate) text_renderer: glyphon::TextRenderer,

    pub default: Font,
}

#[derive(Debug)]
// rather than storing colors directly, we store a key to them for garbage collection purposes
pub struct Font {
    pub names: Vec<String>,
    pub size: u32,
    pub bold: bool,
    pub italic: bool,
    pub color: ColorKey,

    pub shadow: bool,
    pub outline: bool,
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

        let default = Font::default(arenas);

        Self {
            font_system,
            swash_cache,

            cache,
            atlas,
            text_renderer,

            default,
        }
    }
}

impl Font {
    pub fn default(arenas: &mut Arenas) -> Self {
        let color = arenas.colors.insert(Color::WHITE);
        let out_color = arenas.colors.insert(Color::GREY);

        Font {
            names: vec!["Arial".to_string()],
            size: 22,
            bold: false,
            italic: false,
            color,

            shadow: false,
            outline: false,
            out_color,
        }
    }

    pub fn new(
        default: &Self,
        arenas: &mut Arenas,
        names: Option<Vec<String>>,
        size: Option<u32>,
    ) -> Self {
        let names = names.unwrap_or_else(|| default.names.clone());
        let size = size.unwrap_or(default.size);

        let color = arenas.colors[default.color];
        let color = arenas.colors.insert(color);

        let out_color = arenas.colors[default.out_color];
        let out_color = arenas.colors.insert(out_color);

        Self {
            names,
            size,
            color,
            out_color,
            ..*default
        }
    }
}

impl FontKey {
    pub fn set_all_from(self, arenas: &mut Arenas, other: FontKey) {
        let [this, other] = arenas.fonts.get_disjoint_mut([self, other]).unwrap();

        // these are straight copies
        this.names = other.names.clone();
        this.size = other.size;
        this.bold = other.bold;
        this.italic = other.italic;
        this.shadow = other.shadow;
        this.outline = other.outline;

        // these aren't
        arenas.colors[this.color] = arenas.colors[other.color];
        arenas.colors[this.out_color] = arenas.colors[other.out_color];
    }
}
