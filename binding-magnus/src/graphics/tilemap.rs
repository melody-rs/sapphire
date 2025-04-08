use magnus::{Class, Module, Object, method, typed_data::Obj};

use crate::{bind_prop, data::RbTable, def_stubbed_prop};

use super::{RbBitmap, RbViewport};

#[derive(Default)]
#[magnus::wrap(class = "Tilemap", size, free_immediately)]
pub struct Tilemap;

impl Tilemap {
    fn initialize(rb_self: Obj<Self>, viewport: &RbViewport) -> magnus::error::Result<()> {
        let ruby = magnus::Ruby::get_with(rb_self);
        let ary = ruby.ary_from_iter(std::iter::repeat_n(ruby.qnil(), 7));
        rb_self.ivar_set("autotiles", ary)?;

        Ok(())
    }

    fn update(&self) {}

    fn autotiles(rb_self: Obj<Self>) -> magnus::error::Result<magnus::Value> {
        rb_self.ivar_get("autotiles")
    }

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

    def_stubbed_prop!(wrapping -> bool);
    def_stubbed_prop!(ox -> i32);
    def_stubbed_prop!(oy -> i32);

    fn dispose(&self) {}
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Tilemap", ruby.class_object())?;
    class.define_alloc_func::<Tilemap>();

    class.define_method("initialize", method!(Tilemap::initialize, 1))?;

    class.define_method("update", method!(Tilemap::update, 0))?;

    class.define_method("autotiles", method!(Tilemap::autotiles, 0))?;

    bind_prop!(class.tileset = Tilemap::tileset, Tilemap::set_tileset);
    bind_prop!(class.map_data = Tilemap::map_data, Tilemap::set_map_data);
    bind_prop!(
        class.priorities = Tilemap::priorities,
        Tilemap::set_priorities
    );
    bind_prop!(class.wrapping = Tilemap::wrapping, Tilemap::set_wrapping);
    bind_prop!(class.ox = Tilemap::ox, Tilemap::set_ox);
    bind_prop!(class.oy = Tilemap::oy, Tilemap::set_oy);

    class.define_method("dispose", method!(Tilemap::dispose, 0))?;

    Ok(())
}
