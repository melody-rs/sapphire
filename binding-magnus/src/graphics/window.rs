use magnus::{Class, Module, Object, method, typed_data::Obj};
use std::cell::Cell;

use super::{RbBitmap, RbViewport};
use crate::{arenas, data::RbRect, graphics};

#[derive(Default)]
#[magnus::wrap(class = "Window", size, free_immediately)]
pub struct Window(pub Cell<rgss::WindowKey>);

impl Window {
    fn initialize(this: Obj<Self>, args: &[magnus::Value]) -> Result<(), magnus::Error> {
        let graphics = graphics::get().read();
        let mut arenas = arenas::get().write();

        let args = magnus::scan_args::scan_args::<(), _, (), (), (), ()>(args)?;
        let (viewport,): (Option<&RbViewport>,) = args.optional;

        let viewport_key = viewport
            .map(|v| v.0.get())
            .unwrap_or(graphics.global_viewport);

        let window = rgss::Window::new(&mut arenas, viewport_key);
        let z = window.z;

        let wrapped_rect: RbRect = window.rect.into();
        this.ivar_set("wrapped_rect", wrapped_rect)?;

        let wrapped_cursor_rect: RbRect = window.cursor_rect.into();
        this.ivar_set("wrapped_cursor_rect", wrapped_cursor_rect)?;

        let window_key = arenas.windows.insert(window);

        arenas.viewports[viewport_key]
            .z_list
            .insert(z, rgss::Drawable::Window(window_key));

        this.0.set(window_key);

        Ok(())
    }

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
        let graphics = graphics::get().read();
        let mut arenas = arenas::get().write();

        let bitmap = rb_bitmap.map(|b| b.0.get());

        let window_key = rb_self.0.get();
        let window = &mut arenas.windows[window_key];

        match bitmap {
            Some(b) => {
                window.windowskin = bitmap;
                rb_self.ivar_set("windowskin", rb_bitmap)
            }
            None => {
                window.windowskin = bitmap;
                rb_self.ivar_set("windowskin", ruby.qnil())
            }
        }
    }
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Window", ruby.class_object())?;
    class.define_alloc_func::<Window>();
    class.define_method("initialize", method!(Window::initialize, -1))?;

    class.define_method("windowskin", method!(Window::windowskin, 0))?;
    class.define_method("windowskin=", method!(Window::set_windowskin, 1))?;

    Ok(())
}
