use magnus::{Class, Module, Object, method, typed_data::Obj};
use std::cell::Cell;

use super::{RbBitmap, RbViewport};
use crate::{
    arenas,
    data::{RbRect, RbTone},
};

#[derive(Default)]
#[magnus::wrap(class = "Sprite", size, free_immediately)]
pub struct Sprite(pub Cell<rgss::SpriteKey>);

impl Drop for Sprite {
    fn drop(&mut self) {
        let mut arenas = crate::arenas::get().write();
        if arenas.sprites.remove(self.0.get()).is_none() {
            log::warn!("Sprite {:p} was drop'd twice!", self as *mut _)
        }
    }
}

impl Sprite {
    fn initialize(rb_self: Obj<Self>, args: &[magnus::Value]) -> Result<(), magnus::Error> {
        let args = magnus::scan_args::scan_args::<(), _, (), (), (), ()>(args)?;
        let (viewport,): (Option<&RbViewport>,) = args.optional;

        let mut arenas = arenas::get().write();
        let sprite = rgss::Sprite::new(&mut arenas);

        let wrapped_src_rect: RbRect = sprite.src_rect.into();
        rb_self.ivar_set("src_rect", wrapped_src_rect)?;

        let wrapped_tone: RbTone = sprite.tone.into();
        rb_self.ivar_set("tone", wrapped_tone)?;

        let sprite_key = arenas.sprites.insert(sprite);
        rb_self.0.set(sprite_key);

        Ok(())
    }

    fn update(&self) {}

    fn bitmap(rb_self: Obj<Self>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("bitmap")
    }

    fn set_bitmap(rb_self: Obj<Self>, bitmap: Option<Obj<RbBitmap>>) -> magnus::error::Result<()> {
        rb_self.ivar_set("bitmap", bitmap)
    }

    fn pattern(rb_self: Obj<Self>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("pattern")
    }

    fn set_pattern(rb_self: Obj<Self>, bitmap: Option<Obj<RbBitmap>>) -> magnus::error::Result<()> {
        rb_self.ivar_set("pattern", bitmap)
    }

    fn src_rect(rb_self: Obj<Self>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("src_rect")
    }

    fn set_src_rect(rb_self: Obj<Self>, bitmap: Option<Obj<RbRect>>) {}

    fn tone(rb_self: Obj<Self>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("tone")
    }

    fn set_tone(rb_self: Obj<Self>, bitmap: Option<Obj<RbTone>>) {}

    fn visible(&self) -> bool {
        true
    }

    fn set_visible(&self, mirror: bool) {}

    fn mirror(&self) -> bool {
        false
    }

    fn set_mirror(&self, mirror: bool) {}

    fn vmirror(&self) -> bool {
        false
    }

    fn set_vmirror(&self, mirror: bool) {}

    fn zoom_x(&self) -> f32 {
        0.0
    }

    fn set_zoom_x(&self, zoom: f32) {}

    fn zoom_y(&self) -> f32 {
        0.0
    }

    fn set_zoom_y(&self, zoom: f32) {}

    fn ox(&self) -> i32 {
        0
    }

    fn set_ox(&self, offset: i32) {}

    fn oy(&self) -> i32 {
        0
    }

    fn set_oy(&self, offset: i32) {}

    fn blend_type(&self) -> i32 {
        0
    }

    fn set_blend_type(&self, offset: i32) {}

    fn bush_depth(&self) -> i32 {
        0
    }

    fn set_bush_depth(&self, offset: i32) {}

    fn opacity(&self) -> i32 {
        0
    }

    fn set_opacity(&self, opacity: i32) {}

    fn x(&self) -> i32 {
        0
    }

    fn set_x(&self, x: i32) {}

    fn y(&self) -> i32 {
        0
    }

    fn set_y(&self, y: i32) {}

    fn z(&self) -> i32 {
        0
    }

    fn set_z(&self, z: i32) {}

    fn dispose(&self) {}
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Sprite", ruby.class_object())?;
    class.define_alloc_func::<Sprite>();

    class.define_method("initialize", method!(Sprite::initialize, -1))?;

    class.define_method("update", method!(Sprite::update, 0))?;

    class.define_method("bitmap", method!(Sprite::bitmap, 0))?;
    class.define_method("bitmap=", method!(Sprite::set_bitmap, 1))?;

    class.define_method("pattern", method!(Sprite::pattern, 0))?;
    class.define_method("pattern=", method!(Sprite::set_pattern, 1))?;

    class.define_method("src_rect", method!(Sprite::src_rect, 0))?;
    class.define_method("src_rect=", method!(Sprite::set_src_rect, 1))?;

    class.define_method("tone", method!(Sprite::tone, 0))?;
    class.define_method("tone=", method!(Sprite::set_tone, 1))?;

    class.define_method("visible", method!(Sprite::visible, 0))?;
    class.define_method("visible=", method!(Sprite::set_visible, 1))?;

    class.define_method("mirror", method!(Sprite::mirror, 0))?;
    class.define_method("mirror=", method!(Sprite::set_mirror, 1))?;

    class.define_method("vmirror", method!(Sprite::vmirror, 0))?;
    class.define_method("vmirror=", method!(Sprite::set_vmirror, 1))?;

    class.define_method("zoom_x", method!(Sprite::zoom_x, 0))?;
    class.define_method("zoom_x=", method!(Sprite::set_zoom_x, 1))?;

    class.define_method("zoom_y", method!(Sprite::zoom_y, 0))?;
    class.define_method("zoom_y=", method!(Sprite::set_zoom_y, 1))?;

    class.define_method("ox", method!(Sprite::ox, 0))?;
    class.define_method("ox=", method!(Sprite::set_ox, 1))?;

    class.define_method("oy", method!(Sprite::oy, 0))?;
    class.define_method("oy=", method!(Sprite::set_oy, 1))?;

    class.define_method("blend_type", method!(Sprite::blend_type, 0))?;
    class.define_method("blend_type=", method!(Sprite::set_blend_type, 1))?;

    class.define_method("bush_depth", method!(Sprite::bush_depth, 0))?;
    class.define_method("bush_depth=", method!(Sprite::set_bush_depth, 1))?;

    class.define_method("opacity", method!(Sprite::opacity, 0))?;
    class.define_method("opacity=", method!(Sprite::set_opacity, 1))?;

    class.define_method("x", method!(Sprite::x, 0))?;
    class.define_method("x=", method!(Sprite::set_x, 1))?;

    class.define_method("y", method!(Sprite::y, 0))?;
    class.define_method("y=", method!(Sprite::set_y, 1))?;

    class.define_method("z", method!(Sprite::z, 0))?;
    class.define_method("z=", method!(Sprite::set_z, 1))?;

    class.define_method("dispose", method!(Sprite::dispose, 0))?;

    Ok(())
}
