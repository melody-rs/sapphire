use magnus::{Class, Module, Object, method, typed_data::Obj};
use std::cell::Cell;

use super::{RbBitmap, RbViewport};
use crate::{AsKey, arenas, bind_prop, data::RbRect, def_stubbed_prop};

#[derive(Default)]
#[magnus::wrap(class = "Window", size, free_immediately)]
pub struct Window(pub Cell<rgss::WindowKey>);

impl Drop for Window {
    fn drop(&mut self) {
        let mut arenas = crate::arenas::get().write();
        if arenas.windows.remove(self.0.get()).is_none() {
            log::warn!(
                "Window {:p}:{:?} was drop'd twice!",
                self as *mut _,
                self.as_key()
            )
        }
    }
}

impl AsKey for Window {
    type Key = rgss::WindowKey;

    fn as_key(&self) -> Self::Key {
        self.0.get()
    }
}

impl Window {
    fn initialize(this: Obj<Self>, args: &[magnus::Value]) -> Result<(), magnus::Error> {
        let args = magnus::scan_args::scan_args::<(), _, (), (), (), ()>(args)?;
        let (viewport,) = args.optional;
        let viewport: Option<&RbViewport> = viewport.flatten();

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

    def_stubbed_prop!(active -> bool);
    def_stubbed_prop!(visible: true -> bool);
    def_stubbed_prop!(pause -> bool);
    def_stubbed_prop!(x -> i32);
    def_stubbed_prop!(y -> i32);
    def_stubbed_prop!(width: 640 -> i32);
    def_stubbed_prop!(height: 480 -> i32);
    def_stubbed_prop!(z -> i32);
    def_stubbed_prop!(opacity -> i32);
    def_stubbed_prop!(back_opacity -> i32);
    def_stubbed_prop!(contents_opacity -> i32);

    fn dispose(&self) {}
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Window", ruby.class_object())?;
    class.define_alloc_func::<Window>();
    class.define_method("initialize", method!(Window::initialize, -1))?;

    class.define_method("update", method!(Window::update, 0))?;

    bind_prop!(
        class.windowskin = Window::windowskin,
        Window::set_windowskin
    );
    bind_prop!(class.contents = Window::contents, Window::set_contents);
    bind_prop!(
        class.cursor_rect = Window::cursor_rect,
        Window::set_cursor_rect
    );
    bind_prop!(class.active = Window::active, Window::set_active);
    bind_prop!(class.pause = Window::pause, Window::set_pause);
    bind_prop!(class.visible = Window::visible, Window::set_visible);
    bind_prop!(class.x = Window::x, Window::set_x);
    bind_prop!(class.y = Window::y, Window::set_y);
    bind_prop!(class.width = Window::width, Window::set_width);
    bind_prop!(class.height = Window::height, Window::set_height);
    bind_prop!(class.z = Window::z, Window::set_z);
    bind_prop!(class.opacity = Window::opacity, Window::set_opacity);
    bind_prop!(
        class.back_opacity = Window::back_opacity,
        Window::set_back_opacity
    );
    bind_prop!(
        class.contents_opacity = Window::contents_opacity,
        Window::set_contents_opacity
    );

    class.define_method("dispose", method!(Window::dispose, 0))?;

    Ok(())
}
