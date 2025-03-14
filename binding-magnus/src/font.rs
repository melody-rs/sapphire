use magnus::{Object, TryConvert, function, method, value::ReprValue};
use parking_lot::RwLock;
use std::sync::OnceLock;

use crate::data::RbColor;

static FONTS: OnceLock<RwLock<rgss::Fonts>> = OnceLock::new();

pub fn get() -> &'static RwLock<rgss::Fonts> {
    FONTS.get().unwrap()
}

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
    let names = collect_names(arg)?;
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

pub fn bind(ruby: &magnus::Ruby, fonts: rgss::Fonts) -> magnus::error::Result<()> {
    if FONTS.set(RwLock::new(fonts)).is_err() {
        eprintln!("fonts static already set! this is not supposed to happen");
        std::process::abort();
    }

    let class = ruby.define_class("Font", ruby.class_object())?;

    initialize_class_vars(class)?;

    class.define_singleton_method("default_name", method!(default_name, 0))?;
    class.define_singleton_method("default_name=", method!(set_default_name, 1))?;

    class.define_singleton_method("default_size", function!(default_size, 0))?;
    class.define_singleton_method("default_size=", function!(set_default_size, 1))?;

    class.define_singleton_method("default_shadow", function!(default_shadow, 0))?;
    class.define_singleton_method("default_shadow=", function!(set_default_shadow, 1))?;

    Ok(())
}
