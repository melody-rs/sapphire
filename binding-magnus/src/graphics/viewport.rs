use magnus::{Class, Module, Object, TryConvert, Value, method, typed_data::Obj};
use std::cell::Cell;

use crate::{arenas, data::RbRect, graphics};

#[derive(Default)]
#[magnus::wrap(class = "Viewport", size, free_immediately)]
pub struct Viewport(pub Cell<rgss::ViewportKey>);

impl Viewport {
    fn initialize(this: Obj<Self>, args: &[Value]) -> Result<(), magnus::Error> {
        let graphics = graphics::get().read();
        let mut arenas = arenas::get().write();

        let rect = match *args {
            [rect] => {
                let rect: &RbRect = TryConvert::try_convert(rect)?;
                arenas.rects[rect.0.get()]
            }
            [x, y, width, height] => {
                let x = TryConvert::try_convert(x)?;
                let y = TryConvert::try_convert(y)?;
                let width = TryConvert::try_convert(width)?;
                let height = TryConvert::try_convert(height)?;

                rgss::Rect::new(x, y, width, height)
            }
            _ => unreachable!(),
        };

        // TODO move this code to rgss mostly
        let viewport = rgss::Viewport::new(rect, &mut arenas);

        let wrapped_rect: RbRect = viewport.rect.into();
        this.ivar_set("wrapped_rect", wrapped_rect)?;

        let z = viewport.z;
        let viewport_key = arenas.viewports.insert(viewport);

        let global_viewport = &mut arenas.viewports[graphics.global_viewport];
        global_viewport
            .z_list
            .insert(z, rgss::Drawable::Viewport(viewport_key));

        this.0.set(viewport_key);

        Ok(())
    }

    fn z(&self) -> Result<i32, magnus::Error> {
        let arenas = arenas::get().read();
        let viewport_key = self.0.get();
        let z = arenas.viewports[viewport_key].z;
        Ok(z.value())
    }

    fn set_z(&self, z: i32) -> Result<(), magnus::Error> {
        let mut arenas = arenas::get().write();
        let graphics = graphics::get().read();
        let viewport_key = self.0.get();

        let viewport = &mut arenas.viewports[viewport_key];
        let old_z = viewport.z;
        let new_z = old_z.update_value(z);
        viewport.z = new_z;

        let global_viewport = &mut arenas.viewports[graphics.global_viewport];
        global_viewport.z_list.re_insert(old_z, new_z);

        Ok(())
    }
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Viewport", ruby.class_object())?;
    class.define_alloc_func::<Viewport>();
    class.define_method("initialize", method!(Viewport::initialize, -1))?;

    class.define_method("z", method!(Viewport::z, 0))?;
    class.define_method("z=", method!(Viewport::set_z, 1))?;

    Ok(())
}
