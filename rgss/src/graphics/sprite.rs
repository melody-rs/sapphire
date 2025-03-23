use crate::{Arenas, Rect, RectKey, Tone, ToneKey};

pub struct Sprite {
    pub src_rect: RectKey,
    pub tone: ToneKey,
}

impl Sprite {
    pub fn new(arenas: &mut Arenas) -> Self {
        let src_rect = arenas.rects.insert(Rect::default());
        let tone = arenas.tones.insert(Tone::default());

        Self { src_rect, tone }
    }
}
