use magnus::{Class, Module, Object, method, typed_data::Obj};

use crate::data::RbTable;

use super::{RbBitmap, RbViewport};

#[derive(Default)]
#[magnus::wrap(class = "Tilemap", size, free_immediately)]
pub struct Tilemap;

impl Tilemap {
    fn initialize(rb_self: Obj<Self>, viewport: &RbViewport) {}

    fn update(&self) {}

    fn tileset(rb_self: Obj<Self>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("tileset")
    }

    fn set_tileset(rb_self: Obj<Self>, bitmap: Option<Obj<RbBitmap>>) -> magnus::error::Result<()> {
        rb_self.ivar_set("tileset", bitmap)
    }

    fn map_data(rb_self: Obj<Self>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("map_data")
    }

    fn set_map_data(rb_self: Obj<Self>, table: Option<Obj<RbTable>>) -> magnus::error::Result<()> {
        rb_self.ivar_set("map_data", table)
    }

    fn priorities(rb_self: Obj<Self>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("priorities")
    }

    fn set_priorities(
        rb_self: Obj<Self>,
        table: Option<Obj<RbTable>>,
    ) -> magnus::error::Result<()> {
        rb_self.ivar_set("priorities", table)
    }

    fn wrapping(&self) -> bool {
        false
    }

    fn set_wrapping(&self, value: bool) {}

    fn ox(&self) -> i32 {
        0
    }

    fn set_ox(&self, offset: i32) {}

    fn oy(&self) -> i32 {
        0
    }

    fn set_oy(&self, offset: i32) {}
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Tilemap", ruby.class_object())?;
    class.define_alloc_func::<Tilemap>();

    class.define_method("initialize", method!(Tilemap::initialize, 1))?;

    class.define_method("update", method!(Tilemap::update, 0))?;

    class.define_method("tileset", method!(Tilemap::tileset, 0))?;
    class.define_method("tileset=", method!(Tilemap::set_tileset, 1))?;

    class.define_method("map_data", method!(Tilemap::map_data, 0))?;
    class.define_method("map_data=", method!(Tilemap::set_map_data, 1))?;

    class.define_method("priorities", method!(Tilemap::priorities, 0))?;
    class.define_method("priorities=", method!(Tilemap::set_priorities, 1))?;

    class.define_method("wrapping", method!(Tilemap::wrapping, 0))?;
    class.define_method("wrapping=", method!(Tilemap::set_wrapping, 1))?;

    class.define_method("ox", method!(Tilemap::ox, 0))?;
    class.define_method("ox=", method!(Tilemap::set_ox, 1))?;

    class.define_method("oy", method!(Tilemap::oy, 0))?;
    class.define_method("oy=", method!(Tilemap::set_oy, 1))?;

    Ok(())
}
