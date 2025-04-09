#![allow(unused_variables)] // temporarily for now
use std::thread::JoinHandle;

use magnus::{Class, error::ErrorType, function, value::ReprValue};

rb_sys::set_global_tracking_allocator!();

pub fn start(ctx: rgss::Ctx) -> JoinHandle<()> {
    std::thread::Builder::new()
        .name("sapphire main ruby thread".to_owned())
        .spawn(move || {
            // Safety:
            // we're calling this as high up the stack as we feasibly can (right at the start of a new thread's execution!)
            // these bindings don't provide a way to access ruby values *at all* so it's not possible to access ruby values outside of this thread.
            let cleanup = unsafe { magnus::embed::init() };
            let result = run_ruby_thread(&cleanup, ctx);

            input::get().write().exit();

            if let Err(e) = result {
                let exception = match e.error_type() {
                    ErrorType::Jump(_) => None, // ???
                    ErrorType::Error(exception_class, cow) => {
                        log::error!("Rust panicked!");
                        exception_class
                            .new_instance((cow.as_ref(),))
                            .inspect_err(|_| {
                                eprintln!("{exception_class}: {cow}");
                                eprintln!("(failed to properly create an exception)")
                            })
                            .ok()
                    }
                    ErrorType::Exception(exception) => Some(*exception),
                };
                if let Some(exception) = exception {
                    // classname is unsafe because ruby can modify the string it returns.
                    // we're only briefly using it to print the classname though, so its not an issue.
                    eprintln!("{}: {}", unsafe { exception.classname() }, exception);
                    if let Ok(Some(backtrace)) =
                        exception.funcall::<_, _, Option<magnus::RArray>>("backtrace", ())
                    {
                        for line in backtrace {
                            eprintln!("- {}", line);
                        }
                    }
                }
            }
        })
        .expect("failed to start ruby thread")
}

pub struct Script {
    pub name: String,
    pub text: String,
}

impl<'de> alox_48::Deserialize<'de> for Script {
    fn deserialize<D>(deserializer: D) -> alox_48::DeResult<Self>
    where
        D: alox_48::DeserializerTrait<'de>,
    {
        struct Visitor;

        impl<'de> alox_48::de::Visitor<'de> for Visitor {
            type Value = Script;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an array")
            }

            fn visit_array<A>(self, mut array: A) -> alox_48::DeResult<Self::Value>
            where
                A: alox_48::ArrayAccess<'de>,
            {
                use std::io::Read;
                if array.len() != 3 {
                    let error =
                        alox_48::DeError::invalid_length(array.len(), &"an array of length 3");
                    return Err(error);
                }

                let _ = array.next_element::<alox_48::de::Ignored>()?.unwrap();
                let name = array.next_element()?.unwrap();
                let data = array.next_element::<alox_48::RbString>()?.unwrap();

                let mut decoder = flate2::bufread::ZlibDecoder::new(data.as_slice());
                let mut text = String::new();
                decoder
                    .read_to_string(&mut text)
                    .map_err(alox_48::DeError::custom)?;

                Ok(Script { name, text })
            }
        }

        deserializer.deserialize(Visitor)
    }
}

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

mod props;
pub(crate) use props::*;

pub use font::Font as RbFont;

pub trait AsKey {
    type Key: rgss::ArenaKey;
    fn as_key(&self) -> Self::Key;
}

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

const MODULE_RPG: &str = include_str!("rpg/xp.rb");

fn run_ruby_thread(ruby: &magnus::Ruby, ctx: rgss::Ctx) -> magnus::error::Result<()> {
    // VERY important to do this first!!!
    arenas::init(ctx.arenas);

    ruby.define_variable("$debug", cfg!(debug_assertions))?;

    // these two come first as they are really important
    error::bind(ruby)?;
    data::bind(ruby)?;

    // these dont matter as much
    audio::bind(ruby, ctx.audio)?;
    graphics::bind(ruby, ctx.graphics)?;
    input::bind(ruby, ctx.input)?;
    filesystem::bind(ruby, ctx.filesystem)?;
    font::bind(ruby, ctx.fonts)?;
    oneshot::bind(ruby)?;
    modshot::bind(ruby)?;

    {
        fn allow_force_quit() {}
        let module = ruby.define_module("MKXP")?;
        module.define_module_function("allow_force_quit", function!(allow_force_quit, 0))?;
    }

    // after we've finished doing all the bindings, eval the rpg module
    ruby.script("<internal RPG module>");
    ruby.eval::<magnus::Value>(MODULE_RPG)?;

    ruby.eval::<magnus::Value>(
        "$LOAD_PATH.unshift(File.join(Dir.pwd, 'lib', 'ruby'))\n
         $LOAD_PATH.unshift(File.join(Dir.pwd, 'lib', 'ruby', RUBY_PLATFORM))\n",
    )?;

    let script_data = std::fs::read("Data/xScripts.rxdata").unwrap();
    let scripts: Vec<Script> = alox_48::from_bytes(&script_data).unwrap();

    // run all scripts. due to the design of rgss, this will block until script completion
    for script in scripts {
        ruby.script(script.name);
        let result = ruby.eval::<magnus::Value>(&script.text);

        match result {
            Ok(_) => {}
            // if the event loop has exited, the next call to Input::update will raise SystemExit.
            // when we get a systemexit it means the program is exiting so we break out of this loop and stop ruby execution
            Err(error) if error.is_kind_of(ruby.exception_system_exit()) => break,
            Err(error) => return Err(error),
        }
    }

    Ok(())
}
