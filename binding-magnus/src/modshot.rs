mod window;

mod audio_effects;

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    window::bind(ruby)?;
    audio_effects::bind(ruby)?;

    Ok(())
}
