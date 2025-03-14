use convert_case::{Case, Casing};
use magnus::{Module, function};
use parking_lot::RwLock;
use std::sync::OnceLock;

use strum::IntoEnumIterator;

static INPUT: OnceLock<RwLock<rgss::Input>> = OnceLock::new();

pub fn get() -> &'static RwLock<rgss::Input> {
    INPUT.get().unwrap()
}

fn update() -> Result<(), magnus::Error> {
    let mut input = get().write();
    input.update();

    if input.exited() {
        Err(magnus::Error::new(magnus::exception::system_exit(), " "))
    } else {
        Ok(())
    }
}

pub fn bind(ruby: &magnus::Ruby, input: rgss::Input) -> magnus::error::Result<()> {
    if INPUT.set(RwLock::new(input)).is_err() {
        eprintln!("input static already set! this is not supposed to happen");
        std::process::abort();
    }

    let module = ruby.define_module("Input")?;

    for keycode in rgss::KeyCode::iter() {
        let variant_name: &'static str = keycode.into();
        let upper_snake_cased = variant_name.to_case(Case::UpperSnake);
        module.const_set(upper_snake_cased, keycode as u16)?;
    }

    module.define_module_function("update", function!(update, 0))?;

    Ok(())
}
