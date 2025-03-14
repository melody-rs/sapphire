use crate::{Arenas, BitmapKey, Rect, RectKey, ViewportKey};

use super::{Graphics, Z};

pub struct Window {
    pub rect: RectKey,
    pub cursor_rect: RectKey,
    pub active: bool,
    pub contents_opacity: u8,

    pub viewport: ViewportKey,
    pub z: Z,

    pub windowskin: Option<BitmapKey>,
    pub contents: Option<BitmapKey>,
}

impl Window {
    pub fn new(arenas: &mut Arenas, viewport: ViewportKey) -> Self {
        let rect = arenas.rects.insert(Rect::default());
        let cursor_rect = arenas.rects.insert(Rect::default());

        let z = Z::new(0);

        Self {
            rect,
            cursor_rect,
            active: false,
            windowskin: None,
            contents: None,
            contents_opacity: 255,
            viewport,
            z,
        }
    }
}
