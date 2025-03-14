use magnus::Class;

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("RGSSError", ruby.exception_exception().as_r_class())?;

    Ok(())
}
