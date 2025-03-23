use crate::{Arenas, Rect, RectKey};

pub struct Window {
    pub rect: RectKey,
    pub cursor_rect: RectKey,
}

impl Window {
    pub fn new(arenas: &mut Arenas) -> Self {
        let rect = arenas.rects.insert(Rect::default());
        let cursor_rect = arenas.rects.insert(Rect::default());

        Self { rect, cursor_rect }
    }
}
