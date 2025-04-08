use magnus::{Class, Module, Object, TryConvert, Value, method, typed_data::Obj};
use std::cell::Cell;

use crate::{
    AsKey, arenas,
    data::{RbColor, RbRect, RbTone},
    graphics,
};

#[derive(Default)]
#[magnus::wrap(class = "Viewport", size, free_immediately)]
pub struct Viewport(pub Cell<rgss::ViewportKey>);

impl AsKey for Viewport {
    type Key = rgss::ViewportKey;

    fn as_key(&self) -> Self::Key {
        self.0.get()
    }
}

impl Viewport {
    fn initialize(this: Obj<Self>, args: &[Value]) -> Result<(), magnus::Error> {
        let graphics = graphics::get().read();
        let mut arenas = arenas::get().write();

        let rect = match *args {
            [rect] => {
                let rect: &RbRect = TryConvert::try_convert(rect)?;
                arenas[rect.as_key()]
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
        this.ivar_set("rect", wrapped_rect)?;

        let wrapped_tone: RbTone = viewport.tone.into();
        this.ivar_set("tone", wrapped_tone)?;

        let wrapped_color: RbColor = viewport.color.into();
        this.ivar_set("color", wrapped_color)?;

        let z = viewport.z;
        let viewport_key = arenas.viewports.insert(viewport);

        let global_viewport = &mut arenas[graphics.global_viewport];
        global_viewport
            .z_list
            .insert(z, rgss::Drawable::Viewport(viewport_key));

        this.0.set(viewport_key);

        Ok(())
    }

    fn tone(rb_self: Obj<Self>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("tone")
    }

    fn set_tone(rb_self: Obj<Self>, bitmap: Option<Obj<RbTone>>) {}

    fn color(rb_self: Obj<Self>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("color")
    }

    fn set_color(rb_self: Obj<Self>, bitmap: Option<Obj<RbColor>>) {}

    fn z(&self) -> Result<i32, magnus::Error> {
        let arenas = arenas::get().read();
        let viewport_key = self.0.get();
        let z = arenas[viewport_key].z;
        Ok(z.value())
    }

    fn set_z(&self, z: i32) -> Result<(), magnus::Error> {
        let mut arenas = arenas::get().write();
        let graphics = graphics::get().read();
        let viewport_key = self.0.get();

        let viewport = &mut arenas[viewport_key];
        let old_z = viewport.z;
        let new_z = old_z.update_value(z);
        viewport.z = new_z;

        let global_viewport = &mut arenas[graphics.global_viewport];
        global_viewport.z_list.re_insert(old_z, new_z);

        Ok(())
    }

    fn update(&self) {}

    fn dispose(&self) {}

    fn ox(&self) -> i32 {
        0
    }

    fn set_ox(&self, offset: i32) {}

    fn oy(&self) -> i32 {
        0
    }

    fn set_oy(&self, offset: i32) {}

    fn visible(&self) -> bool {
        true
    }

    fn set_visible(&self, mirror: bool) {}
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Viewport", ruby.class_object())?;
    class.define_alloc_func::<Viewport>();
    class.define_method("initialize", method!(Viewport::initialize, -1))?;

    class.define_method("update", method!(Viewport::update, 0))?;

    class.define_method("tone", method!(Viewport::tone, 0))?;
    class.define_method("tone=", method!(Viewport::set_tone, 1))?;

    class.define_method("color", method!(Viewport::color, 0))?;
    class.define_method("color=", method!(Viewport::set_color, 1))?;

    class.define_method("visible", method!(Viewport::visible, 0))?;
    class.define_method("visible=", method!(Viewport::set_visible, 1))?;

    class.define_method("z", method!(Viewport::z, 0))?;
    class.define_method("z=", method!(Viewport::set_z, 1))?;

    class.define_method("dispose", method!(Viewport::dispose, 0))?;

    class.define_method("ox", method!(Viewport::ox, 0))?;
    class.define_method("ox=", method!(Viewport::set_ox, 1))?;

    class.define_method("oy", method!(Viewport::oy, 0))?;
    class.define_method("oy=", method!(Viewport::set_oy, 1))?;

    Ok(())
}
