use magnus::{Class, Module, TryConvert, Value, method};
use std::cell::Cell;

use crate::{
    arenas,
    data::{RbColor, RbRect},
    filesystem, graphics,
};

#[derive(Default)]
#[magnus::wrap(class = "Bitmap", size, free_immediately)]
pub struct Bitmap(pub Cell<rgss::BitmapKey>);

impl Drop for Bitmap {
    fn drop(&mut self) {
        let mut arenas = crate::arenas::get().write();
        if arenas.bitmaps.remove(self.0.get()).is_none() {
            log::warn!("Bitmap {:p} was drop'd twice!", self as *mut _)
        }
    }
}

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

    fn width(&self) -> u32 {
        let arenas = arenas::get().read();
        arenas.bitmaps[self.0.get()].width()
    }

    fn height(&self) -> u32 {
        let arenas = arenas::get().read();
        arenas.bitmaps[self.0.get()].height()
    }

    fn clear(&self) {}

    fn blt(&self, args: &[Value]) -> magnus::error::Result<()> {
        magnus::scan_args::check_arity(args.len(), 4..=5)?;

        let [x, y, src, src_rect] = args[..4] else {
            unreachable!()
        };
        let x = i32::try_convert(x)?;
        let y = i32::try_convert(y)?;
        let src: &Self = TryConvert::try_convert(src)?;
        let src_rect: &RbRect = TryConvert::try_convert(src_rect)?;

        let opacity = args
            .get(5)
            .copied()
            .map(TryConvert::try_convert)
            .transpose()?
            .unwrap_or(255);

        Ok(())
    }

    fn stretch_blt(&self, args: &[Value]) -> magnus::error::Result<()> {
        magnus::scan_args::check_arity(args.len(), 3..=4)?;

        let [dst_rect, src, src_rect] = args[..3] else {
            unreachable!()
        };
        let dst_rect: &RbRect = TryConvert::try_convert(dst_rect)?;
        let src: &Self = TryConvert::try_convert(src)?;
        let src_rect: &RbRect = TryConvert::try_convert(src_rect)?;

        let opacity = args
            .get(4)
            .copied()
            .map(TryConvert::try_convert)
            .transpose()?
            .unwrap_or(255);

        Ok(())
    }

    fn fill_rect(&self, args: &[Value]) -> magnus::error::Result<()> {
        magnus::scan_args::check_arity(args.len(), 2..=5)?;

        match *args {
            [rect, color] => {
                let rb_rect: &RbRect = TryConvert::try_convert(rect)?;
                let color: &RbColor = TryConvert::try_convert(color)?;
            }
            [x, y, width, height, color] => {
                let x = i32::try_convert(x)?;
                let y = i32::try_convert(y)?;
                let width = u32::try_convert(width)?;
                let height = u32::try_convert(height)?;
                let color: &RbColor = TryConvert::try_convert(color)?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn dispose(&self) {}
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Bitmap", ruby.class_object())?;
    class.define_alloc_func::<Bitmap>();
    class.define_method("initialize", method!(Bitmap::initialize, -1))?;

    class.define_method("width", method!(Bitmap::width, 0))?;
    class.define_method("height", method!(Bitmap::height, 0))?;

    class.define_method("clear", method!(Bitmap::clear, 0))?;

    class.define_method("blt", method!(Bitmap::blt, -1))?;
    class.define_method("stretch_blt", method!(Bitmap::stretch_blt, -1))?;
    class.define_method("fill_rect", method!(Bitmap::fill_rect, -1))?;

    class.define_method("dispose", method!(Bitmap::dispose, 0))?;

    Ok(())
}
