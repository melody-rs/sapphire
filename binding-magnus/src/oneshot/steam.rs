use magnus::{Module, function};

fn enabled() -> bool {
    false
}

fn unlock(ach: String) {}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let module = ruby.define_module("Steam")?;

    module.define_module_function("enabled?", function!(enabled, 0))?;
    module.define_module_function("unlock", function!(unlock, 1))?;

    module.const_set("LANG", "en")?;

    Ok(())
}
