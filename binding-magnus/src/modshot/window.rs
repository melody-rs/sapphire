use crate::graphics;
use magnus::function;

fn set_title(title: String) {
    let graphics = graphics::get().read();
    let window = graphics.main_window();
    window.set_title(&title);
}

fn set_icon(icon: String) {}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let module = ruby.define_module("ModWindow")?;

    module.define_module_function("SetTitle", function!(set_title, 1))?;
    module.define_module_function("SetIcon", function!(set_icon, 1))?;

    Ok(())
}
