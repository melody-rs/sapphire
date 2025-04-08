use magnus::{Class, Module, Value, method};
use std::cell::Cell;

use crate::{AsKey, arenas, bind_prop, def_val_prop};

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
        if arenas.rects.remove(self.as_key()).is_none() {
            log::warn!(
                "Rect {:p}:{:?} was drop'd twice!",
                self as *mut _,
                self.as_key()
            )
        }
    }
}

impl AsKey for Rect {
    type Key = rgss::RectKey;

    fn as_key(&self) -> Self::Key {
        self.0.get()
    }
}

impl Rect {
    pub fn new(arenas: &mut rgss::Arenas, x: i32, y: i32, width: u32, height: u32) -> Self {
        let mut arenas = arenas::get().write();
        let rect = rgss::Rect {
            x,
            y,
            width,
            height,
        };
        let rect_key = arenas.rects.insert(rect);
        rect_key.into()
    }

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

    fn initialize_copy(&self, other: &Self) {
        let mut arenas = arenas::get().write();
        let copy = arenas[other.as_key()];
        let key = arenas.rects.insert(copy);
        self.0.set(key);
    }

    def_val_prop!(x -> i32);
    def_val_prop!(y -> i32);
    def_val_prop!(width -> u32);
    def_val_prop!(height -> u32);

    fn set(&self, x: i32, y: i32, width: u32, height: u32) {
        let mut arenas = arenas::get().write();
        let rect = &mut arenas[self.as_key()];
        *rect = rgss::Rect {
            x,
            y,
            width,
            height,
        }
    }

    fn empty(&self) {
        let mut arenas = arenas::get().write();
        let rect = &mut arenas[self.as_key()];
        *rect = rgss::Rect::default();
    }
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Rect", ruby.class_object())?;
    class.define_alloc_func::<Rect>();
    class.define_method("initialize", method!(Rect::initialize, -1))?;
    class.define_method("initialize_copy", method!(Rect::initialize_copy, 1))?;

    bind_prop!(class.x = Rect::x, Rect::set_x);
    bind_prop!(class.y = Rect::y, Rect::set_y);
    bind_prop!(class.width = Rect::width, Rect::set_width);
    bind_prop!(class.height = Rect::height, Rect::set_height);

    class.define_method("set", method!(Rect::set, 4))?;
    class.define_method("empty", method!(Rect::empty, 0))?;

    Ok(())
}
