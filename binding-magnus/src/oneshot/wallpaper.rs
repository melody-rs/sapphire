use magnus::function;

fn reset() {}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let module = ruby.define_module("Wallpaper")?;

    module.define_module_function("reset", function!(reset, 0))?;

    Ok(())
}
