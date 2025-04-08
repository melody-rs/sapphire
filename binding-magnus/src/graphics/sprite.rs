use magnus::{Class, Module, Object, method, typed_data::Obj};
use std::cell::Cell;

use super::{RbBitmap, RbViewport};
use crate::{
    AsKey, arenas, bind_prop,
    data::{RbRect, RbTone},
    def_stubbed_prop,
};

#[derive(Default)]
#[magnus::wrap(class = "Sprite", size, free_immediately)]
pub struct Sprite(pub Cell<rgss::SpriteKey>);

impl Drop for Sprite {
    fn drop(&mut self) {
        let mut arenas = crate::arenas::get().write();
        if arenas.sprites.remove(self.0.get()).is_none() {
            log::warn!(
                "Sprite {:p}:{:?} was drop'd twice!",
                self as *mut _,
                self.as_key()
            )
        }
    }
}

impl AsKey for Sprite {
    type Key = rgss::SpriteKey;

    fn as_key(&self) -> Self::Key {
        self.0.get()
    }
}

impl Sprite {
    fn initialize(rb_self: Obj<Self>, args: &[magnus::Value]) -> Result<(), magnus::Error> {
        let args = magnus::scan_args::scan_args::<(), _, (), (), (), ()>(args)?;
        let (viewport,) = args.optional;
        let viewport: Option<&RbViewport> = viewport.flatten();

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

    def_stubbed_prop!(visible -> bool);
    def_stubbed_prop!(mirror -> bool);
    def_stubbed_prop!(vmirror -> bool);
    def_stubbed_prop!(zoom_x -> f32);
    def_stubbed_prop!(zoom_y -> f32);
    def_stubbed_prop!(angle -> f32);
    def_stubbed_prop!(ox -> i32);
    def_stubbed_prop!(oy -> i32);
    def_stubbed_prop!(blend_type -> i32);
    def_stubbed_prop!(pattern_blend_type -> i32);
    def_stubbed_prop!(bush_depth -> i32);
    def_stubbed_prop!(opacity -> i32);
    def_stubbed_prop!(x -> i32);
    def_stubbed_prop!(y -> i32);
    def_stubbed_prop!(z -> i32);

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

    bind_prop!(class.visible = Sprite::visible, Sprite::set_visible);
    bind_prop!(class.mirror = Sprite::mirror, Sprite::set_mirror);
    bind_prop!(class.vmirror = Sprite::vmirror, Sprite::set_vmirror);
    bind_prop!(class.zoom_x = Sprite::zoom_x, Sprite::set_zoom_x);
    bind_prop!(class.zoom_y = Sprite::zoom_y, Sprite::set_zoom_y);
    bind_prop!(class.angle = Sprite::angle, Sprite::set_angle);
    bind_prop!(class.ox = Sprite::ox, Sprite::set_ox);
    bind_prop!(class.oy = Sprite::oy, Sprite::set_oy);
    bind_prop!(
        class.blend_type = Sprite::blend_type,
        Sprite::set_blend_type
    );
    bind_prop!(
        class.pattern_blend_type = Sprite::pattern_blend_type,
        Sprite::set_pattern_blend_type
    );
    bind_prop!(
        class.bush_depth = Sprite::bush_depth,
        Sprite::set_bush_depth
    );
    bind_prop!(class.opacity = Sprite::opacity, Sprite::set_opacity);
    bind_prop!(class.x = Sprite::x, Sprite::set_x);
    bind_prop!(class.y = Sprite::y, Sprite::set_y);
    bind_prop!(class.z = Sprite::z, Sprite::set_z);

    class.define_method("dispose", method!(Sprite::dispose, 0))?;

    Ok(())
}
