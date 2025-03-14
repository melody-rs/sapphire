mod window;

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    window::bind(ruby)?;

    Ok(())
}
