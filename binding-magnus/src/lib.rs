#![allow(unused_variables)] // temporarily for now

rb_sys::set_global_tracking_allocator!();

mod audio;
mod data;
mod error;
mod filesystem;
mod font;
mod graphics;
mod gvl;
mod input;
mod modshot;
mod oneshot;

mod bindings;
pub use bindings::Magnus;

mod props;
pub(crate) use props::*;

pub use font::Font as RbFont;

mod arenas {
    use parking_lot::RwLock;
    use std::sync::OnceLock;

    // FIXME find a way around using a static
    pub(crate) static ARENAS: OnceLock<RwLock<rgss::Arenas>> = OnceLock::new();

    #[inline(always)]
    pub fn get() -> &'static RwLock<rgss::Arenas> {
        ARENAS
            .get()
            .expect("arenas static not set! please report how you encountered this crash")
    }

    pub fn init(arenas: rgss::Arenas) {
        // panic if arenas is set! this should not *ever* happen
        if ARENAS.set(RwLock::new(arenas)).is_err() {
            eprintln!("arenas static already set! this is not supposed to happen");
            std::process::abort();
        }
    }
}

pub trait AsKey {
    type Key: rgss::ArenaKey;
    fn as_key(&self) -> Self::Key;
}
