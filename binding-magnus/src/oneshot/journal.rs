use magnus::function;

fn set(name: String) {}

fn active() -> bool {
    false
}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let module = ruby.define_module("Journal")?;

    module.define_module_function("set", function!(set, 1))?;
    module.define_module_function("active?", function!(active, 0))?;

    Ok(())
}
