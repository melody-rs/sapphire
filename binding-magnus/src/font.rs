use magnus::{
    Class, Module, Object, TryConvert, function, method, typed_data::Obj, value::ReprValue,
};
use parking_lot::RwLock;
use std::{cell::Cell, sync::OnceLock};

use crate::{arenas, data::RbColor};

static FONTS: OnceLock<RwLock<rgss::Fonts>> = OnceLock::new();

pub fn get() -> &'static RwLock<rgss::Fonts> {
    FONTS.get().unwrap()
}

#[derive(Default)]
#[magnus::wrap(class = "Font", size, free_immediately)]
pub struct Font(pub Cell<rgss::FontKey>);

impl Drop for Font {
    fn drop(&mut self) {
        let mut arenas = crate::arenas::get().write();
        if arenas.fonts.remove(self.0.get()).is_none() {
            log::warn!("Font {:p} was drop'd twice!", self as *mut _)
        }
    }
}

impl Font {
    fn collect_names(value: magnus::Value) -> magnus::error::Result<Vec<String>> {
        let ruby = magnus::Ruby::get_with(value);

        if value.is_kind_of(ruby.class_array()) {
            let names = Vec::<String>::try_convert(value)?;
            Ok(names)
        } else if value.is_kind_of(ruby.class_string()) {
            let name = String::try_convert(value)?;
            Ok(vec![name])
        } else {
            Ok(vec![])
        }
    }

    fn initialize_class_vars(class: magnus::RClass) -> magnus::error::Result<()> {
        let fonts = get().read();

        match fonts.default.names.as_slice() {
            [name] => class.ivar_set("default_name", name.as_str())?,
            names => {
                let ary = magnus::RArray::from_iter(names.iter().map(String::as_str));
                class.ivar_set("default_name", ary)?
            }
        }

        // create a ruby color object using the default color and set it for garbage collection reasons
        let rb_color = RbColor::from(fonts.default.color);
        class.ivar_set("default_color", rb_color)?;

        Ok(())
    }

    fn default_name(class: magnus::RClass) -> Result<magnus::Value, magnus::Error> {
        class.ivar_get("default_name")
    }

    fn set_default_name(class: magnus::RClass, arg: magnus::Value) -> Result<(), magnus::Error> {
        let names = Self::collect_names(arg)?;
        let mut fonts = get().write();
        fonts.default.names = names;

        class.ivar_set("name", arg)?;

        Ok(())
    }

    fn default_size() -> u32 {
        get().read().default.size
    }

    fn set_default_size(size: u32) {
        get().write().default.size = size
    }

    fn default_shadow() -> bool {
        get().read().default.shadow
    }

    fn set_default_shadow(shadow: bool) {
        get().write().default.shadow = shadow
    }

    fn initialize(rb_self: Obj<Self>, args: &[magnus::Value]) -> magnus::error::Result<()> {
        magnus::scan_args::check_arity(args.len(), 0..=2)?;

        let fonts = get().read();
        let mut arenas = arenas::get().write();

        let default_name = rb_self.class().ivar_get("default_name")?;
        let names_obj = args.first().copied().unwrap_or(default_name);

        let names = args.first().copied().map(Self::collect_names).transpose()?;
        let size = args
            .get(1)
            .copied()
            .map(TryConvert::try_convert)
            .transpose()?;

        let font = rgss::Font::new(&fonts.default, &mut arenas, names, size);
        let font_key = arenas.fonts.insert(font);

        rb_self.0.set(font_key);

        Ok(())
    }
}

pub fn bind(ruby: &magnus::Ruby, fonts: rgss::Fonts) -> magnus::error::Result<()> {
    if FONTS.set(RwLock::new(fonts)).is_err() {
        eprintln!("fonts static already set! this is not supposed to happen");
        std::process::abort();
    }

    let class = ruby.define_class("Font", ruby.class_object())?;
    class.define_alloc_func::<Font>();
    class.define_method("initialize", method!(Font::initialize, -1))?;

    Font::initialize_class_vars(class)?;

    class.define_singleton_method("default_name", method!(Font::default_name, 0))?;
    class.define_singleton_method("default_name=", method!(Font::set_default_name, 1))?;

    class.define_singleton_method("default_size", function!(Font::default_size, 0))?;
    class.define_singleton_method("default_size=", function!(Font::set_default_size, 1))?;

    class.define_singleton_method("default_shadow", function!(Font::default_shadow, 0))?;
    class.define_singleton_method("default_shadow=", function!(Font::set_default_shadow, 1))?;

    Ok(())
}
