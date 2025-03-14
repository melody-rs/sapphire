use crate::{Arenas, Rect, RectKey};

use super::z::{Z, ZList};

pub struct Viewport {
    // TODO avoid storing Z twice (once in zlist, once here)
    pub z: Z,
    pub rect: RectKey,
    pub z_list: ZList,
}

impl Viewport {
    pub fn new(rect: Rect, arenas: &mut Arenas) -> Self {
        let rect_key = arenas.rects.insert(rect);
        Self {
            z: Z::new(0),
            rect: rect_key,
            z_list: ZList::new(),
        }
    }

    pub(crate) fn global(arenas: &mut Arenas) -> Self {
        let rect = Rect::new(0, 0, 640, 480);
        let rect_key = arenas.rects.insert(rect);
        Self {
            z: Z::new(0),
            rect: rect_key,
            z_list: ZList::new(),
        }
    }
}
