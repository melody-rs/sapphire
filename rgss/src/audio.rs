pub struct Audio {
    manager: kira::AudioManager,
}

type Error = kira::backend::cpal::Error;

impl Audio {
    pub fn new() -> Result<Self, Error> {
        let settings = kira::AudioManagerSettings::default();
        let manager = kira::AudioManager::new(settings)?;
        Ok(Self { manager })
    }
}
