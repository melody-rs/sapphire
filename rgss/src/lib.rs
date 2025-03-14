mod event_loop;
pub use event_loop::{EventLoop, Events};

mod graphics;
pub use graphics::{Bitmap, Drawable, Graphics, Viewport, Window};

mod input;
pub use input::{Input, KeyCode};

mod audio;
pub use audio::Audio;

mod filesystem;
pub use filesystem::FileSystem;

mod font;
pub use font::{Font, Fonts};

mod data;
pub use data::{Color, Rect, Table, Tone};
use slotmap::{SlotMap, new_key_type};

// Arenas is a workaround for garbage collection, and to keep everything in one convenient place.
// rgss is dependent on global state, which is one of the main reasons we need this struct.
// when rendering, we have to traverse the viewport tree which means looking at *every* single sprite
// it keeps everything cache local (more or less) and fast to access.
//
// the other is for object properties (like a font's color, or a sprite's bitmap).
// we *have* to store an object, but the rgss crate doesn't *know* about objects.
// so instead, we store an index into the slotmap for them instead, and create an object that "owns" the slotmap key.
// this means that object properties have an independent lifetime to the thing they are stored in, which is exactly what we want for garbage collection.
//
// the alternative would be using Arc + Weak everywhere (ew, no thanks!)
#[derive(Default)]
pub struct Arenas {
    pub fonts: SlotMap<FontKey, Font>,
    pub colors: SlotMap<ColorKey, Color>,
    pub tones: SlotMap<ToneKey, Tone>,
    pub rects: SlotMap<RectKey, Rect>,
    pub tables: SlotMap<TableKey, Table>,
    pub bitmaps: SlotMap<BitmapKey, Bitmap>,
    pub viewports: SlotMap<ViewportKey, Viewport>,
    pub windows: SlotMap<WindowKey, Window>,
}

new_key_type! {
    pub struct FontKey;
    pub struct ColorKey;
    pub struct ToneKey;
    pub struct RectKey;
    pub struct TableKey;
    pub struct BitmapKey;
    pub struct ViewportKey;
    pub struct WindowKey;
}

pub struct Ctx {
    pub arenas: Arenas,

    pub audio: Audio,
    pub graphics: Graphics,
    pub input: Input,
    pub filesystem: FileSystem,
    pub fonts: Fonts,
}
