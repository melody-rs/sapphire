use magnus::{Class, Module, Object, method, typed_data::Obj};

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

    fn z(&self) -> i32 {
        0
    }

    fn set_z(&self, z: i32) {}

    fn dispose(&self) {}
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Plane", ruby.class_object())?;
    class.define_alloc_func::<Plane>();

    class.define_method("initialize", method!(Plane::initialize, 1))?;

    class.define_method("bitmap", method!(Plane::bitmap, 0))?;
    class.define_method("bitmap=", method!(Plane::set_bitmap, 1))?;

    class.define_method("zoom_x", method!(Plane::zoom_x, 0))?;
    class.define_method("zoom_x=", method!(Plane::set_zoom_x, 1))?;

    class.define_method("zoom_y", method!(Plane::zoom_y, 0))?;
    class.define_method("zoom_y=", method!(Plane::set_zoom_y, 1))?;

    class.define_method("ox", method!(Plane::ox, 0))?;
    class.define_method("ox=", method!(Plane::set_ox, 1))?;

    class.define_method("oy", method!(Plane::oy, 0))?;
    class.define_method("oy=", method!(Plane::set_oy, 1))?;

    class.define_method("z", method!(Plane::z, 0))?;
    class.define_method("z=", method!(Plane::set_z, 1))?;

    class.define_method("dispose", method!(Plane::dispose, 0))?;

    Ok(())
}
