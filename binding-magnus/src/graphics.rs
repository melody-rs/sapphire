use magnus::function;
use parking_lot::RwLock;
use std::sync::OnceLock;

mod bitmap;
pub use bitmap::Bitmap as RbBitmap;

mod plane;
mod sprite;
mod tilemap;
mod viewport;
pub use viewport::Viewport as RbViewport;

mod window;

static GRAPHICS: OnceLock<RwLock<rgss::Graphics>> = OnceLock::new();

pub fn get() -> &'static RwLock<rgss::Graphics> {
    GRAPHICS.get().unwrap()
}

fn fullscreen() -> bool {
    false
}

fn set_fullscreen(fullscreen: bool) {}

fn frame_rate() -> u16 {
    60
}

fn set_frame_rate(framerate: u16) {}

fn frame_count() -> u64 {
    0
}

fn set_frame_count(count: u64) {}

fn update() {}

fn frameskip() -> bool {
    false
}

fn set_frameskip(frameskip: bool) {}

fn frame_reset() {}

fn transition(args: &[magnus::Value]) -> Result<(), magnus::Error> {
    let args = magnus::scan_args::scan_args::<(), _, (), (), (), ()>(args)?;

    let (duration, filename, vague): (Option<u32>, Option<String>, Option<bool>) = args.optional;

    Ok(())
}

pub fn bind(ruby: &magnus::Ruby, graphics: rgss::Graphics) -> magnus::error::Result<()> {
    if GRAPHICS.set(RwLock::new(graphics)).is_err() {
        eprintln!("graphics static already set! this is not supposed to happen");
        std::process::abort();
    }

    let module = ruby.define_module("Graphics")?;

    sprite::bind(ruby)?;
    bitmap::bind(ruby)?;
    window::bind(ruby)?;
    tilemap::bind(ruby)?;
    plane::bind(ruby)?;
    viewport::bind(ruby)?;

    module.define_module_function("update", function!(update, 0))?;
    module.define_module_function("frame_reset", function!(frame_reset, 0))?;
    module.define_module_function("transition", function!(transition, -1))?;

    module.define_module_function("fullscreen", function!(fullscreen, 0))?;
    module.define_module_function("fullscreen=", function!(set_fullscreen, 1))?;

    module.define_module_function("frame_rate", function!(frame_rate, 0))?;
    module.define_module_function("frame_rate=", function!(set_frame_rate, 1))?;

    module.define_module_function("frame_count", function!(frame_count, 0))?;
    module.define_module_function("frame_count=", function!(set_frame_count, 1))?;

    module.define_module_function("frameskip", function!(frameskip, 0))?;
    module.define_module_function("frameskip=", function!(set_frameskip, 1))?;

    Ok(())
}
