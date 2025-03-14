use magnus::{Class, Module, Object, RString, Value, function, method};
use std::cell::Cell;

use crate::arenas;

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
        if arenas.colors.remove(self.0.get()).is_none() {
            log::warn!("Color {:p} was drop'd twice!", self as *mut _)
        }
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

    fn serialize(color: &Color) -> RString {
        let arenas = arenas::get().read();
        let color = arenas.colors[color.0.get()];
        let bytes = bytemuck::bytes_of(&color);
        RString::from_slice(bytes)
    }
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Color", ruby.class_object())?;
    class.define_alloc_func::<Color>();
    class.define_method("initialize", method!(Color::initialize, -1))?;
    class.define_singleton_method("_load", function!(Color::deserialize, 1))?;
    class.define_method("_dump_data", method!(Color::serialize, 0))?;

    Ok(())
}
