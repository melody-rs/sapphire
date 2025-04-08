mod event_loop;
pub use event_loop::{EventLoop, Events};

mod graphics;
pub use graphics::{Bitmap, Drawable, Graphics, Sprite, Viewport, Window};

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

mod config;
pub use config::Config;

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
    pub sprites: SlotMap<SpriteKey, Sprite>,
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
    pub struct SpriteKey;
}

pub struct Ctx {
    pub arenas: Arenas,

    pub audio: Audio,
    pub graphics: Graphics,
    pub input: Input,
    pub filesystem: FileSystem,
    pub fonts: Fonts,
}

pub trait ArenaKey: slotmap::Key {
    type Value;

    fn slotmap(arenas: &Arenas) -> &slotmap::SlotMap<Self, Self::Value>;
    fn slotmap_mut(arenas: &mut Arenas) -> &mut slotmap::SlotMap<Self, Self::Value>;
}

macro_rules! impl_key_for {
    ($type:ty => $value:ty : $field:ident) => {
        impl ArenaKey for $type {
            type Value = $value;
            fn slotmap(arenas: &Arenas) -> &slotmap::SlotMap<Self, Self::Value> {
                &arenas.$field
            }
            fn slotmap_mut(arenas: &mut Arenas) -> &mut slotmap::SlotMap<Self, Self::Value> {
                &mut arenas.$field
            }
        }
    };
}

impl_key_for!(FontKey => Font: fonts);
impl_key_for!(ColorKey => Color: colors);
impl_key_for!(ToneKey => Tone: tones);
impl_key_for!(RectKey => Rect: rects);
impl_key_for!(TableKey => Table: tables);
impl_key_for!(BitmapKey => Bitmap: bitmaps);
impl_key_for!(ViewportKey => Viewport: viewports);
impl_key_for!(WindowKey => Window: windows);
impl_key_for!(SpriteKey => Sprite: sprites);

impl<T> std::ops::Index<T> for Arenas
where
    T: ArenaKey + 'static,
{
    type Output = T::Value;

    fn index(&self, index: T) -> &Self::Output {
        &T::slotmap(self)[index]
    }
}

impl<T> std::ops::IndexMut<T> for Arenas
where
    T: ArenaKey + 'static,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        &mut T::slotmap_mut(self)[index]
    }
}
