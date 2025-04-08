use magnus::{Class, Module, Object, RString, Value, function, method};
use std::cell::Cell;

use crate::{AsKey, arenas, bind_prop, def_val_prop};

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
            log::warn!(
                "Tone {:p}:{:?} was drop'd twice!",
                self as *mut _,
                self.as_key()
            )
        }
    }
}

impl AsKey for Tone {
    type Key = rgss::ToneKey;

    fn as_key(&self) -> Self::Key {
        self.0.get()
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

    fn initialize_copy(&self, other: &Self) {
        let mut arenas = arenas::get().write();
        let copy = arenas[other.as_key()];
        let key = arenas.tones.insert(copy);
        self.0.set(key);
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

    fn serialize(tone: &Tone, _: i32) -> RString {
        let arenas = arenas::get().read();
        let tone = arenas[tone.as_key()];
        let bytes = bytemuck::bytes_of(&tone);
        RString::from_slice(bytes)
    }

    def_val_prop!(red -> f64);
    def_val_prop!(green -> f64);
    def_val_prop!(blue -> f64);
    def_val_prop!(gray -> f64);

    fn set(&self, red: f64, blue: f64, green: f64, gray: f64) {
        let mut arenas = arenas::get().write();
        let tone = &mut arenas[self.as_key()];
        *tone = rgss::Tone {
            red,
            blue,
            green,
            gray,
        }
    }

    fn empty(&self) {
        let mut arenas = arenas::get().write();
        let tone = &mut arenas[self.as_key()];
        *tone = rgss::Tone::default();
    }
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Tone", ruby.class_object())?;
    class.define_alloc_func::<Tone>();
    class.define_method("initialize", method!(Tone::initialize, -1))?;
    class.define_method("initialize_copy", method!(Tone::initialize_copy, 1))?;
    class.define_singleton_method("_load", function!(Tone::deserialize, 1))?;
    class.define_method("_dump", method!(Tone::serialize, 1))?;

    bind_prop!(class.red = Tone::red, Tone::set_red);
    bind_prop!(class.green = Tone::green, Tone::set_green);
    bind_prop!(class.blue = Tone::blue, Tone::set_blue);
    bind_prop!(class.gray = Tone::gray, Tone::set_gray);

    class.define_method("set", method!(Tone::set, 4))?;
    class.define_method("empty", method!(Tone::empty, 0))?;

    Ok(())
}
