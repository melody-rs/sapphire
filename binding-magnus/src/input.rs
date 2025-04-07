use magnus::{Module, function};
use parking_lot::RwLock;
use std::sync::OnceLock;

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

fn triggered(key: i32) -> bool {
    false
}

fn pressed(key: i32) -> bool {
    false
}

fn repeat(key: i32) -> bool {
    false
}

fn triggerex(key: magnus::Symbol) -> bool {
    false
}

fn pressex(key: magnus::Symbol) -> bool {
    false
}

fn repeatex(key: magnus::Symbol) -> bool {
    false
}

pub fn bind(ruby: &magnus::Ruby, input: rgss::Input) -> magnus::error::Result<()> {
    if INPUT.set(RwLock::new(input)).is_err() {
        eprintln!("input static already set! this is not supposed to happen");
        std::process::abort();
    }

    let module = ruby.define_module("Input")?;

    module.define_module_function("update", function!(update, 0))?;

    module.define_module_function("trigger?", function!(triggered, 1))?;
    module.define_module_function("press?", function!(pressed, 1))?;
    module.define_module_function("repeat?", function!(repeat, 1))?;

    module.define_module_function("triggerex?", function!(triggerex, 1))?;
    module.define_module_function("pressex?", function!(pressex, 1))?;
    module.define_module_function("repeatex?", function!(repeatex, 1))?;

    Ok(())
}
