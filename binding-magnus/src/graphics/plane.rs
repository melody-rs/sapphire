use magnus::{Class, Module, Object, method, typed_data::Obj};

use crate::{bind_prop, def_stubbed_prop};

use super::{RbBitmap, RbViewport};

#[derive(Default)]
#[magnus::wrap(class = "Plane", size, free_immediately)]
pub struct Plane;

impl Plane {
    fn initialize(rb_self: Obj<Self>, viewport: &RbViewport) {}

    fn bitmap(rb_self: Obj<Self>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("bitmap")
    }

    fn set_bitmap(rb_self: Obj<Self>, bitmap: Option<Obj<RbBitmap>>) -> magnus::error::Result<()> {
        rb_self.ivar_set("bitmap", bitmap)
    }

    def_stubbed_prop!(zoom_x -> f32);
    def_stubbed_prop!(zoom_y -> f32);
    def_stubbed_prop!(ox -> i32);
    def_stubbed_prop!(oy -> i32);
    def_stubbed_prop!(z -> i32);

    fn dispose(&self) {}
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Plane", ruby.class_object())?;
    class.define_alloc_func::<Plane>();

    class.define_method("initialize", method!(Plane::initialize, 1))?;

    class.define_method("bitmap", method!(Plane::bitmap, 0))?;
    class.define_method("bitmap=", method!(Plane::set_bitmap, 1))?;

    bind_prop!(class.zoom_x = Plane::zoom_x, Plane::set_zoom_x);
    bind_prop!(class.zoom_y = Plane::zoom_y, Plane::set_zoom_y);
    bind_prop!(class.ox = Plane::ox, Plane::set_ox);
    bind_prop!(class.oy = Plane::oy, Plane::set_oy);
    bind_prop!(class.z = Plane::z, Plane::set_z);

    class.define_method("dispose", method!(Plane::dispose, 0))?;

    Ok(())
}
