use magnus::{Class, Module, Object, method, typed_data::Obj};
use std::cell::Cell;

use super::{RbBitmap, RbViewport};
use crate::{arenas, data::RbRect};

#[derive(Default)]
#[magnus::wrap(class = "Window", size, free_immediately)]
pub struct Window(pub Cell<rgss::WindowKey>);

impl Drop for Window {
    fn drop(&mut self) {
        let mut arenas = crate::arenas::get().write();
        if arenas.windows.remove(self.0.get()).is_none() {
            log::warn!("Window {:p} was drop'd twice!", self as *mut _)
        }
    }
}

impl Window {
    fn initialize(this: Obj<Self>, args: &[magnus::Value]) -> Result<(), magnus::Error> {
        let args = magnus::scan_args::scan_args::<(), _, (), (), (), ()>(args)?;
        let (viewport,): (Option<&RbViewport>,) = args.optional;

        let mut arenas = arenas::get().write();

        let window = rgss::Window::new(&mut arenas);

        let wrapped_rect: RbRect = window.rect.into();
        this.ivar_set("rect", wrapped_rect)?;

        let wrapped_cursor_rect: RbRect = window.cursor_rect.into();
        this.ivar_set("cursor_rect", wrapped_cursor_rect)?;

        let window_key = arenas.windows.insert(window);
        this.0.set(window_key);

        Ok(())
    }

    fn update(&self) {}

    fn cursor_rect(rb_self: Obj<Window>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("cursor_rect")
    }

    fn set_cursor_rect(rb_self: Obj<Window>, rb_bitmap: Obj<RbRect>) {}

    fn windowskin(rb_self: Obj<Window>) -> Result<magnus::Value, magnus::Error> {
        // we fetch the ivar so we don't return multiple Bitmap objects.
        // ruby.
        rb_self.ivar_get("windowskin")
    }

    fn set_windowskin(
        rb_self: Obj<Window>,
        rb_bitmap: Option<Obj<RbBitmap>>,
    ) -> Result<(), magnus::Error> {
        let ruby = magnus::Ruby::get_with(rb_self);
        let bitmap = rb_bitmap.map(|b| b.0.get());

        match bitmap {
            Some(b) => rb_self.ivar_set("windowskin", rb_bitmap),
            None => rb_self.ivar_set("windowskin", ruby.qnil()),
        }
    }

    fn contents(rb_self: Obj<Window>) -> Result<magnus::Value, magnus::Error> {
        // we fetch the ivar so we don't return multiple Bitmap objects.
        // ruby.
        rb_self.ivar_get("contents")
    }

    fn set_contents(
        rb_self: Obj<Window>,
        rb_bitmap: Option<Obj<RbBitmap>>,
    ) -> Result<(), magnus::Error> {
        let ruby = magnus::Ruby::get_with(rb_self);
        let bitmap = rb_bitmap.map(|b| b.0.get());

        match bitmap {
            Some(b) => rb_self.ivar_set("contents", rb_bitmap),
            None => rb_self.ivar_set("contents", ruby.qnil()),
        }
    }

    fn active(&self) -> bool {
        true
    }

    fn set_active(&self, mirror: bool) {}

    fn visible(&self) -> bool {
        true
    }

    fn set_visible(&self, mirror: bool) {}

    fn x(&self) -> i32 {
        0
    }

    fn set_x(&self, x: i32) {}

    fn y(&self) -> i32 {
        0
    }

    fn set_y(&self, y: i32) {}

    fn width(&self) -> i32 {
        640
    }

    fn set_width(&self, y: i32) {}

    fn height(&self) -> i32 {
        480
    }

    fn set_height(&self, y: i32) {}

    fn z(&self) -> i32 {
        0
    }

    fn set_z(&self, z: i32) {}

    fn back_opacity(&self) -> i32 {
        255
    }

    fn set_back_opacity(&self, z: i32) {}
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Window", ruby.class_object())?;
    class.define_alloc_func::<Window>();
    class.define_method("initialize", method!(Window::initialize, -1))?;

    class.define_method("update", method!(Window::update, 0))?;

    class.define_method("windowskin", method!(Window::windowskin, 0))?;
    class.define_method("windowskin=", method!(Window::set_windowskin, 1))?;

    class.define_method("contents", method!(Window::contents, 0))?;
    class.define_method("contents=", method!(Window::set_contents, 1))?;

    class.define_method("cursor_rect", method!(Window::cursor_rect, 0))?;
    class.define_method("cursor_rect=", method!(Window::set_cursor_rect, 1))?;

    class.define_method("active", method!(Window::active, 0))?;
    class.define_method("active=", method!(Window::set_active, 1))?;

    class.define_method("visible", method!(Window::visible, 0))?;
    class.define_method("visible=", method!(Window::set_visible, 1))?;

    class.define_method("x", method!(Window::x, 0))?;
    class.define_method("x=", method!(Window::set_x, 1))?;

    class.define_method("y", method!(Window::y, 0))?;
    class.define_method("y=", method!(Window::set_y, 1))?;

    class.define_method("width", method!(Window::width, 0))?;
    class.define_method("width=", method!(Window::set_width, 1))?;

    class.define_method("height", method!(Window::height, 0))?;
    class.define_method("height=", method!(Window::set_height, 1))?;

    class.define_method("z", method!(Window::z, 0))?;
    class.define_method("z=", method!(Window::set_z, 1))?;

    class.define_method("back_opacity", method!(Window::back_opacity, 0))?;
    class.define_method("back_opacity=", method!(Window::set_back_opacity, 1))?;

    Ok(())
}
