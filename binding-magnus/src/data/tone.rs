use magnus::{Class, Module, Object, RString, Value, function, method};
use std::cell::Cell;

use crate::arenas;

#[derive(Default)]
#[magnus::wrap(class = "Tone", size, free_immediately)]
pub struct Tone(pub Cell<rgss::ToneKey>);

impl From<rgss::ToneKey> for Tone {
    fn from(value: rgss::ToneKey) -> Self {
        Self(Cell::new(value))
    }
}

// removes the tone from arenas.
// this should only happen once, and should only happen when a tone is no longer used!
impl Drop for Tone {
    fn drop(&mut self) {
        let mut arenas = crate::arenas::get().write();
        if arenas.tones.remove(self.0.get()).is_none() {
            log::warn!("Tone {:p} was drop'd twice!", self as *mut _)
        }
    }
}

impl Tone {
    fn initialize(&self, args: &[Value]) -> magnus::error::Result<()> {
        let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

        let (red, green, blue) = args.required;
        let (grey,) = args.optional;

        let mut arenas = arenas::get().write();
        let tone = rgss::Tone {
            red,
            blue,
            green,
            gray: grey.unwrap_or(255.0),
        };
        let tone_key = arenas.tones.insert(tone);

        self.0.set(tone_key);

        Ok(())
    }

    fn red(&self) -> f64 {
        let arenas = arenas::get().read();
        arenas.tones[self.0.get()].red
    }

    fn set_red(&self, value: f64) {
        let mut arenas = arenas::get().write();
        arenas.tones[self.0.get()].red = value;
    }

    fn green(&self) -> f64 {
        let arenas = arenas::get().read();
        arenas.tones[self.0.get()].green
    }

    fn set_green(&self, value: f64) {
        let mut arenas = arenas::get().write();
        arenas.tones[self.0.get()].green = value;
    }

    fn blue(&self) -> f64 {
        let arenas = arenas::get().read();
        arenas.tones[self.0.get()].blue
    }

    fn set_blue(&self, value: f64) {
        let mut arenas = arenas::get().write();
        arenas.tones[self.0.get()].blue = value;
    }

    fn gray(&self) -> f64 {
        let arenas = arenas::get().read();
        arenas.tones[self.0.get()].gray
    }

    fn set_gray(&self, value: f64) {
        let mut arenas = arenas::get().write();
        arenas.tones[self.0.get()].gray = value;
    }

    fn deserialize(bytes: RString) -> Tone {
        let mut arenas = arenas::get().write();
        // We don't hold onto the slice long enough for ruby to do anything with it.
        let tone: rgss::Tone = unsafe {
            let bytes = bytes.as_slice();
            *bytemuck::from_bytes(bytes)
        };
        let tone_key = arenas.tones.insert(tone);
        Self::from(tone_key)
    }

    fn serialize(tone: &Tone) -> RString {
        let arenas = arenas::get().read();
        let tone = arenas.tones[tone.0.get()];
        let bytes = bytemuck::bytes_of(&tone);
        RString::from_slice(bytes)
    }
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Tone", ruby.class_object())?;
    class.define_alloc_func::<Tone>();
    class.define_method("initialize", method!(Tone::initialize, -1))?;
    class.define_singleton_method("_load", function!(Tone::deserialize, 1))?;
    class.define_method("_dump_data", method!(Tone::serialize, 0))?;

    class.define_method("red", method!(Tone::red, 0))?;
    class.define_method("green", method!(Tone::green, 0))?;
    class.define_method("blue", method!(Tone::blue, 0))?;
    class.define_method("gray", method!(Tone::gray, 0))?;

    class.define_method("red=", method!(Tone::set_red, 1))?;
    class.define_method("green=", method!(Tone::set_green, 1))?;
    class.define_method("blue=", method!(Tone::set_blue, 1))?;
    class.define_method("gray=", method!(Tone::set_gray, 1))?;

    Ok(())
}
