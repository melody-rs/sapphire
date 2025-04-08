use magnus::{
    Class, Module, Object, TryConvert, TypedData, method, typed_data::Obj, value::ReprValue,
};
use parking_lot::RwLock;
use std::{cell::Cell, sync::OnceLock};

use crate::{AsKey, arenas, bind_class_prop, bind_prop, data::RbColor, def_val_prop};

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
            log::warn!(
                "Font {:p}:{:?} was drop'd twice!",
                self as *mut _,
                self.as_key()
            )
        }
    }
}

impl AsKey for Font {
    type Key = rgss::FontKey;
    fn as_key(&self) -> Self::Key {
        self.0.get()
    }
}

// class fns
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
}

// methods
impl Font {
    pub fn new_default(
        arenas: &mut rgss::Arenas,
        fonts: &rgss::Fonts,
    ) -> magnus::error::Result<Obj<Self>> {
        let ruby = magnus::Ruby::get().unwrap();

        let default_name: magnus::Value = Font::class(&ruby).ivar_get("default_name")?;

        let font = rgss::Font::new(&fonts.default, arenas, None, None);
        let color: RbColor = font.color.into();
        let out_color: RbColor = font.out_color.into();

        let key = arenas.fonts.insert(font);

        let rb_self = Obj::wrap(Self(Cell::new(key)));
        // see https://github.com/Ancurio/mkxp/blob/master/binding-mri/font-binding.cpp#L94-L96
        rb_self.ivar_set("name", default_name)?;
        rb_self.ivar_set("color", color)?;
        rb_self.ivar_set("out_color", out_color)?;

        Ok(rb_self)
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
        let color: RbColor = font.color.into();
        let out_color: RbColor = font.out_color.into();

        let font_key = arenas.fonts.insert(font);

        rb_self.0.set(font_key);

        // see https://github.com/Ancurio/mkxp/blob/master/binding-mri/font-binding.cpp#L94-L96
        rb_self.ivar_set("name", default_name)?;
        rb_self.ivar_set("color", color)?;
        rb_self.ivar_set("out_color", out_color)?;

        Ok(())
    }

    def_val_prop!(size -> u32);
    def_val_prop!(bold -> bool);
    def_val_prop!(italic -> bool);
    def_val_prop!(shadow -> bool);
    def_val_prop!(outline -> bool);

    // these are more complicated
    fn name(rb_self: Obj<Self>) -> magnus::error::Result<magnus::Value> {
        rb_self.ivar_get("name")
    }

    fn set_name(rb_self: Obj<Self>, val: magnus::Value) -> magnus::error::Result<()> {
        let mut arenas = arenas::get().write();

        let names = Self::collect_names(val)?;
        arenas[rb_self.as_key()].names = names;
        rb_self.ivar_set("name", val)?;

        Ok(())
    }

    fn color(rb_self: Obj<Self>) -> magnus::error::Result<magnus::Value> {
        rb_self.ivar_get("color")
    }

    fn set_color(&self, val: &RbColor) {
        let mut arenas = arenas::get().write();
        let color = arenas[self.as_key()].color;
        arenas[color] = arenas[val.as_key()];
    }

    fn out_color(rb_self: Obj<Self>) -> magnus::error::Result<magnus::Value> {
        rb_self.ivar_get("out_color")
    }

    fn set_out_color(&self, val: &RbColor) {
        let mut arenas = arenas::get().write();
        let out_color = arenas[self.as_key()].out_color;
        arenas[out_color] = arenas[val.as_key()];
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

    // this one is special
    class.define_singleton_method("default_name", method!(Font::default_name, 0))?;
    class.define_singleton_method("default_name=", method!(Font::set_default_name, 1))?;

    bind_class_prop!(
        class.default_size = Font::default_size,
        Font::set_default_size
    );
    bind_class_prop!(
        class.default_shadow = Font::default_shadow,
        Font::set_default_shadow
    );

    bind_prop!(class.name = Font::name, Font::set_name);
    bind_prop!(class.size = Font::size, Font::set_size);
    bind_prop!(class.bold = Font::bold, Font::set_bold);
    bind_prop!(class.italic = Font::italic, Font::set_italic);
    bind_prop!(class.color = Font::color, Font::set_color);
    bind_prop!(class.shadow = Font::shadow, Font::set_shadow);
    bind_prop!(class.outline = Font::outline, Font::set_outline);
    bind_prop!(class.out_color = Font::out_color, Font::set_out_color);

    Ok(())
}
