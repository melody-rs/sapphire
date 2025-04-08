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

use crate::{bind_module_prop, def_stubbed_class_prop};

mod window;

static GRAPHICS: OnceLock<RwLock<rgss::Graphics>> = OnceLock::new();

pub fn get() -> &'static RwLock<rgss::Graphics> {
    GRAPHICS.get().unwrap()
}

def_stubbed_class_prop!(fullscreen -> bool);
def_stubbed_class_prop!(frame_rate: 60 -> u16);
def_stubbed_class_prop!(frame_count -> u64);
def_stubbed_class_prop!(frameskip -> bool);

fn update() {}

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

    bind_module_prop!(module.fullscreen = fullscreen, set_fullscreen);
    bind_module_prop!(module.frame_rate = frame_rate, set_frame_rate);
    bind_module_prop!(module.frame_count = frame_count, set_frame_count);
    bind_module_prop!(module.frameskip = frameskip, set_frameskip);

    Ok(())
}
