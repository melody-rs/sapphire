use magnus::{Value, function};
use parking_lot::RwLock;
use std::sync::OnceLock;

// TODO investigate maybeuninit? i doubt the perf hit of using OnceLock is much though
// TODO maybe use AtomicRefCell (or some equivalent)
static AUDIO: OnceLock<RwLock<rgss::Audio>> = OnceLock::new();

pub fn get() -> &'static RwLock<rgss::Audio> {
    AUDIO.get().unwrap()
}

fn stub(_: &[Value]) {}

pub fn bind(ruby: &magnus::Ruby, audio: rgss::Audio) -> magnus::error::Result<()> {
    if AUDIO.set(RwLock::new(audio)).is_err() {
        eprintln!("audio static already set! this is not supposed to happen");
        std::process::abort();
    }

    let module = ruby.define_module("Audio")?;

    module.define_module_function("bgm_play", function!(stub, -1))?;
    module.define_module_function("bgm_stop", function!(stub, -1))?;
    module.define_module_function("bgm_fade", function!(stub, -1))?;

    module.define_module_function("bgs_play", function!(stub, -1))?;
    module.define_module_function("bgs_stop", function!(stub, -1))?;
    module.define_module_function("bgs_fade", function!(stub, -1))?;

    module.define_module_function("me_play", function!(stub, -1))?;
    module.define_module_function("me_stop", function!(stub, -1))?;
    module.define_module_function("me_fade", function!(stub, -1))?;

    module.define_module_function("se_play", function!(stub, -1))?;
    module.define_module_function("se_stop", function!(stub, -1))?;

    module.define_module_function("ch_play", function!(stub, -1))?;
    module.define_module_function("lch_play", function!(stub, -1))?;

    Ok(())
}
