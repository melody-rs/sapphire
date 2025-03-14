use magnus::{Class, Module, Object, RString, Value, function, method};
use std::cell::Cell;

use crate::arenas;

#[derive(Default)]
#[magnus::wrap(class = "Rect", size, free_immediately)]
pub struct Rect(pub Cell<rgss::RectKey>);

impl From<rgss::RectKey> for Rect {
    fn from(value: rgss::RectKey) -> Self {
        Self(Cell::new(value))
    }
}

// removes the rect from arenas.
// this should only happen once, and should only happen when a rect is no longer used!
impl Drop for Rect {
    fn drop(&mut self) {
        let mut arenas = crate::arenas::get().write();
        if arenas.rects.remove(self.0.get()).is_none() {
            log::warn!("Rect {:p} was drop'd twice!", self as *mut _)
        }
    }
}

impl Rect {
    fn initialize(&self, args: &[Value]) -> magnus::error::Result<()> {
        let args = magnus::scan_args::scan_args::<_, (), (), (), (), ()>(args)?;

        let (x, y, width, height) = args.required;

        let mut arenas = arenas::get().write();
        let rect = rgss::Rect {
            x,
            y,
            width,
            height,
        };
        let rect_key = arenas.rects.insert(rect);

        self.0.set(rect_key);

        Ok(())
    }
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Rect", ruby.class_object())?;
    class.define_alloc_func::<Rect>();
    class.define_method("initialize", method!(Rect::initialize, -1))?;

    Ok(())
}
