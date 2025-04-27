use crate::graphics;
use magnus::function;

fn set_title(title: String) {
    let graphics = graphics::get().read();
    let window = graphics.main_window();
    window.set_title(&title);
}

fn set_icon(icon: String) {
    let icon_data = image::open(icon).unwrap().into_rgba8();
    let width = icon_data.width();
    let height = icon_data.height();
    let icon = winit::window::Icon::from_rgba(icon_data.into_vec(), width, height).unwrap();

    let graphics = graphics::get().read();
    let window = graphics.main_window();
    window.set_window_icon(Some(icon));
}

fn pos_supported() -> bool {
    true
}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let module = ruby.define_module("ModWindow")?;

    module.define_module_function("SetTitle", function!(set_title, 1))?;
    module.define_module_function("SetIcon", function!(set_icon, 1))?;
    module.define_module_function("pos_supported", function!(pos_supported, 0))?;

    Ok(())
}
