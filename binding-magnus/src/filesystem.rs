use magnus::{Module, RModule, Value, function, value::ReprValue};
use std::sync::OnceLock;

// FIXME find a way around using a static
pub(crate) static FILESYSTEM: OnceLock<rgss::FileSystem> = OnceLock::new();

#[inline(always)]
pub fn get() -> &'static rgss::FileSystem {
    FILESYSTEM
        .get()
        .expect("filesystem static not set! please report how you encountered this crash")
}

fn load_data(path: String) -> magnus::error::Result<Value> {
    //? SAFETY
    // This function is only exposed to Ruby. It is not possible to call this without it being called on a Ruby thread
    let ruby = unsafe { magnus::Ruby::get_unchecked() };

    // TODO this does *double* copies! which is bad.
    let filesystem = get();
    // FIXME proper error handling!
    let mut file = filesystem.read_file(path).expect("failed to read file");

    let mut buf = vec![];
    file.read_to_end(&mut buf).expect("failed to read file");

    let ruby_string = ruby.str_from_slice(&buf);

    let marshal: RModule = ruby.module_kernel().const_get("Marshal")?;
    marshal.funcall("load", (ruby_string,))
}

pub fn bind(ruby: &magnus::Ruby, filesystem: rgss::FileSystem) -> Result<(), magnus::Error> {
    // panic if filesysten is set! this should not *ever* happen
    if FILESYSTEM.set(filesystem).is_err() {
        eprintln!("filesystem static already set! this is not supposed to happen");
        std::process::abort();
    }

    let module = ruby.module_kernel();

    module.define_module_function("load_data", function!(load_data, 1))?;

    Ok(())
}
