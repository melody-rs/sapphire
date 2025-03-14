pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Sprite", ruby.class_object())?;

    Ok(())
}
