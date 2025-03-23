use crate::{Arenas, Color, ColorKey, Rect, RectKey, Tone, ToneKey};

use super::z::{Z, ZList};

pub struct Viewport {
    // TODO avoid storing Z twice (once in zlist, once here)
    pub z: Z,
    pub rect: RectKey,
    pub z_list: ZList,
    pub tone: ToneKey,
    pub color: ColorKey,
}

impl Viewport {
    pub fn new(rect: Rect, arenas: &mut Arenas) -> Self {
        let rect_key = arenas.rects.insert(rect);
        let tone = arenas.tones.insert(Tone::default());
        let color = arenas.colors.insert(Color::default());
        Self {
            z: Z::new(0),
            rect: rect_key,
            tone,
            color,
            z_list: ZList::new(),
        }
    }

    pub(crate) fn global(arenas: &mut Arenas) -> Self {
        let rect = Rect::new(0, 0, 640, 480);
        let rect_key = arenas.rects.insert(rect);
        let tone = arenas.tones.insert(Tone::default());
        let color = arenas.colors.insert(Color::default());
        Self {
            z: Z::new(0),
            rect: rect_key,
            tone,
            color,
            z_list: ZList::new(),
        }
    }
}
