use magnus::{Class, Module, TryConvert, Value, method};
use std::cell::Cell;

use crate::{arenas, filesystem, graphics};

#[derive(Default)]
#[magnus::wrap(class = "Bitmap", size, free_immediately)]
pub struct Bitmap(pub Cell<rgss::BitmapKey>);

impl Bitmap {
    fn initialize(&self, args: &[Value]) -> Result<(), magnus::Error> {
        magnus::scan_args::check_arity(args.len(), 1..=2)?;

        let graphics = graphics::get().read();
        let filesystem = filesystem::get();
        let mut arenas = arenas::get().write();

        let bitmap = match *args {
            [path] => {
                let path = String::try_convert(path)?;

                rgss::Bitmap::new_path(&graphics, filesystem, path)
            }
            [width, height] => {
                let width = u32::try_convert(width)?;
                let height = u32::try_convert(height)?;

                rgss::Bitmap::new(&graphics, width, height)
            }
            _ => unreachable!(),
        };

        let bitmap_key = arenas.bitmaps.insert(bitmap);
        self.0.set(bitmap_key);

        Ok(())
    }
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Bitmap", ruby.class_object())?;
    class.define_alloc_func::<Bitmap>();
    class.define_method("initialize", method!(Bitmap::initialize, -1))?;

    Ok(())
}
