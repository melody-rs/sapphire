use magnus::{Class, Module, Object, RString, Value, function, method};
use std::cell::Cell;

use crate::{AsKey, arenas, bind_prop, def_val_prop};

#[derive(Default)]
#[magnus::wrap(class = "Color", size, free_immediately)]
pub struct Color(pub Cell<rgss::ColorKey>);

impl From<rgss::ColorKey> for Color {
    fn from(value: rgss::ColorKey) -> Self {
        Self(Cell::new(value))
    }
}

// removes the color from arenas.
// this should only happen once, and should only happen when a color is no longer used!
impl Drop for Color {
    fn drop(&mut self) {
        let mut arenas = crate::arenas::get().write();
        if arenas.colors.remove(self.as_key()).is_none() {
            log::warn!(
                "Color {:p}:{:?} was drop'd twice!",
                self as *mut _,
                self.as_key()
            )
        }
    }
}

impl AsKey for Color {
    type Key = rgss::ColorKey;

    fn as_key(&self) -> Self::Key {
        self.0.get()
    }
}

impl Color {
    fn initialize(&self, args: &[Value]) -> magnus::error::Result<()> {
        let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

        let (red, green, blue) = args.required;
        let (alpha,) = args.optional;

        let mut arenas = arenas::get().write();
        let color = rgss::Color {
            red,
            blue,
            green,
            alpha: alpha.unwrap_or(255.0),
        };
        let color_key = arenas.colors.insert(color);

        self.0.set(color_key);

        Ok(())
    }

    fn initialize_copy(&self, other: &Self) {
        let mut arenas = arenas::get().write();
        let copy = arenas[other.as_key()];
        let key = arenas.colors.insert(copy);
        self.0.set(key);
    }

    fn deserialize(bytes: RString) -> Color {
        let mut arenas = arenas::get().write();
        // We don't hold onto the slice long enough for ruby to do anything with it.
        let color: rgss::Color = unsafe {
            let bytes = bytes.as_slice();
            *bytemuck::from_bytes(bytes)
        };
        let color_key = arenas.colors.insert(color);
        Self::from(color_key)
    }

    fn serialize(color: &Color, _: i32) -> RString {
        let arenas = arenas::get().read();
        let color = arenas[color.as_key()];
        let bytes = bytemuck::bytes_of(&color);
        RString::from_slice(bytes)
    }

    def_val_prop!(red -> f64);
    def_val_prop!(green -> f64);
    def_val_prop!(blue -> f64);
    def_val_prop!(alpha -> f64);

    fn set(&self, red: f64, blue: f64, green: f64, alpha: f64) {
        let mut arenas = arenas::get().write();
        let color = &mut arenas[self.as_key()];
        *color = rgss::Color {
            red,
            blue,
            green,
            alpha,
        }
    }

    fn empty(&self) {
        let mut arenas = arenas::get().write();
        let color = &mut arenas[self.as_key()];
        *color = rgss::Color::default();
    }
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Color", ruby.class_object())?;
    class.define_alloc_func::<Color>();
    class.define_method("initialize", method!(Color::initialize, -1))?;
    class.define_method("initialize_copy", method!(Color::initialize_copy, 1))?;
    class.define_singleton_method("_load", function!(Color::deserialize, 1))?;
    class.define_method("_dump", method!(Color::serialize, 1))?;

    bind_prop!(class.red = Color::red, Color::set_red);
    bind_prop!(class.green = Color::green, Color::set_green);
    bind_prop!(class.blue = Color::blue, Color::set_blue);
    bind_prop!(class.alpha = Color::alpha, Color::set_alpha);

    class.define_method("set", method!(Color::set, 4))?;
    class.define_method("empty", method!(Color::empty, 0))?;

    Ok(())
}
