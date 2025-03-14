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
            grey: grey.unwrap_or(255.0),
        };
        let tone_key = arenas.tones.insert(tone);

        self.0.set(tone_key);

        Ok(())
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

    Ok(())
}
