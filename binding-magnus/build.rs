fn main() {
    // TODO handle linking ourselves based on pkgconfig, build ruby in build script
    #[cfg(all(unix, feature = "ruby-static"))]
    {
        // This is required because ruby needs us to link against zlib in static builds
        // (Normally libruby.so would link against it and we wouldn't need this)
        // I'd really rather handling linking ourselves, as it's quite complicated and rb-sys doesn't do a great job
        println!("cargo::rustc-link-lib=static=z")
    }
}
