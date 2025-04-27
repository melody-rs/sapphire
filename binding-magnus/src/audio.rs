use magnus::{Value, function};
use parking_lot::Mutex;
use std::sync::OnceLock;

use crate::filesystem;

// TODO investigate maybeuninit? i doubt the perf hit of using OnceLock is much though
// TODO maybe use AtomicRefCell (or some equivalent)
static AUDIO: OnceLock<Mutex<rgss::Audio>> = OnceLock::new();

pub fn get() -> &'static Mutex<rgss::Audio> {
    AUDIO.get().unwrap()
}

fn bgm_play(args: &[Value]) -> magnus::error::Result<()> {
    let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;
    let (name,): (String,) = args.required;
    let (volume, pitch, position) = args.optional;

    let mut audio = get().lock();
    let filesystem = filesystem::get();

    let opts = rgss::PlayOptions {
        name,
        volume,
        pitch,
        position,
    };
    audio.bgm_play(filesystem, opts);

    Ok(())
}

fn bgm_stop() {
    let mut audio = get().lock();
    audio.bgm_stop();
}

fn bgm_pos() -> f64 {
    let audio = get().lock();
    audio.bgm_pos()
}

fn bgs_play(args: &[Value]) -> magnus::error::Result<()> {
    let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;
    let (name,): (String,) = args.required;
    let (volume, pitch, position) = args.optional;

    let mut audio = get().lock();
    let filesystem = filesystem::get();

    let opts = rgss::PlayOptions {
        name,
        volume,
        pitch,
        position,
    };
    audio.bgs_play(filesystem, opts);

    Ok(())
}

fn bgs_stop() {
    let mut audio = get().lock();
    audio.bgs_stop();
}

fn bgs_pos() -> f64 {
    let audio = get().lock();
    audio.bgs_pos()
}

fn se_play(args: &[Value]) -> magnus::error::Result<()> {
    let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;
    let (name,): (String,) = args.required;
    let (volume, pitch, position) = args.optional;

    let mut audio = get().lock();
    let filesystem = filesystem::get();

    let opts = rgss::PlayOptions {
        name,
        volume,
        pitch,
        position,
    };
    audio.se_play(filesystem, opts);

    Ok(())
}

fn se_stop() {
    let mut audio = get().lock();
    audio.se_stop();
}

fn stub(_: &[Value]) {}

pub fn bind(ruby: &magnus::Ruby, audio: rgss::Audio) -> magnus::error::Result<()> {
    if AUDIO.set(Mutex::new(audio)).is_err() {
        eprintln!("audio static already set! this is not supposed to happen");
        std::process::abort();
    }

    let module = ruby.define_module("Audio")?;

    module.define_module_function("bgm_play", function!(bgm_play, -1))?;
    module.define_module_function("bgm_stop", function!(bgm_stop, 0))?;
    module.define_module_function("bgm_fade", function!(stub, -1))?;
    module.define_module_function("bgm_set_al_effect", function!(stub, -1))?;
    module.define_module_function("bgm_pos", function!(bgm_pos, 0))?;

    module.define_module_function("bgs_play", function!(bgs_play, -1))?;
    module.define_module_function("bgs_stop", function!(bgs_stop, 0))?;
    module.define_module_function("bgs_fade", function!(stub, -1))?;
    module.define_module_function("bgs_set_al_effect", function!(stub, -1))?;
    module.define_module_function("bgs_pos", function!(bgs_pos, 0))?;

    module.define_module_function("me_play", function!(stub, -1))?;
    module.define_module_function("me_stop", function!(stub, -1))?;
    module.define_module_function("me_set_al_effect", function!(stub, -1))?;
    module.define_module_function("me_fade", function!(stub, -1))?;

    module.define_module_function("se_play", function!(se_play, -1))?;
    module.define_module_function("se_set_al_effect", function!(stub, -1))?;
    module.define_module_function("se_stop", function!(se_stop, 0))?;

    module.define_module_function("ch_play", function!(stub, -1))?;
    module.define_module_function("lch_play", function!(stub, -1))?;

    Ok(())
}
