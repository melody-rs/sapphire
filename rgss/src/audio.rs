use kira::track::TrackBuilder;

use crate::{FileSystem, filesystem::File};

pub struct Audio {
    manager: kira::AudioManager,

    bgm: Track,
    bgs: Track,

    // se and me are special
    se_track: kira::track::TrackHandle,
    playing_ses: Vec<SoundHandle>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cpal error: {0}")]
    CpalError(#[from] kira::backend::cpal::Error),
    #[error("Kira error {0}")]
    ResourceLimitReached(#[from] kira::ResourceLimitReached),
}

pub struct PlayOptions {
    pub name: String,
    pub volume: Option<u32>,
    pub pitch: Option<u32>,
    pub position: Option<f64>,
}

struct Track {
    track_handle: kira::track::TrackHandle,
    sound: Option<Sound>, // Sound is !Sync so it needs a mutex
}

struct Sound {
    sound_handle: SoundHandle,
    name: String,
}

// holy shit this is a god awful type
type SoundData = kira::sound::streaming::StreamingSoundData<kira::sound::FromFileError>;
type SoundHandle = kira::sound::streaming::StreamingSoundHandle<kira::sound::FromFileError>;

impl Track {
    pub fn new(manager: &mut kira::AudioManager) -> Result<Self, Error> {
        let track_handle = manager.add_sub_track(TrackBuilder::default())?;
        Ok(Self {
            track_handle,
            sound: None,
        })
    }
}

impl Sound {
    pub fn update(
        &mut self,
        volume: kira::Decibels,
        pitch: kira::PlaybackRate,
        position: Option<f64>,
    ) {
        let tween = kira::Tween::default();

        self.sound_handle.set_volume(volume, tween);
        self.sound_handle.set_playback_rate(pitch, tween);
        if let Some(position) = position {
            self.sound_handle.seek_to(position);
        }
    }
}

impl PlayOptions {
    fn position_to_start_time(&self) -> kira::StartTime {
        self.position
            .map(|pos| {
                let dur = std::time::Duration::from_secs_f64(pos);
                kira::StartTime::Delayed(dur)
            })
            .unwrap_or_default()
    }

    fn pitch_to_playback_rate(&self) -> kira::PlaybackRate {
        self.pitch
            .map(|p| p as f64 / 100.0)
            .map(kira::PlaybackRate)
            .unwrap_or_default()
    }

    fn volume_to_db(&self) -> kira::Decibels {
        let amplitude = self.volume.map(|v| v as f32 / 100.0).unwrap_or(1.0);
        // TODO (my hears hurt while figuring this one out lmao, so i need a break)
        // https://github.com/mkxp-z/mkxp-z/pull/208/files#diff-3216992fdc41349399a23a9468d6e272ba8382e89f63d2beebd0d477b468372eR174-R200
        kira::Decibels(amplitude.log10() * 4.0 / 7.0 + 1.0)
    }
}

impl Audio {
    pub fn new() -> Result<Self, Error> {
        let settings = kira::AudioManagerSettings::default();
        let mut manager = kira::AudioManager::new(settings)?;

        let bgm = Track::new(&mut manager)?;
        let bgs = Track::new(&mut manager)?;

        let se_track = manager.add_sub_track(TrackBuilder::default())?;

        Ok(Self {
            manager,
            bgm,
            bgs,
            se_track,
            playing_ses: Vec::with_capacity(10),
        })
    }

    pub fn bgm_play(&mut self, filesystem: &FileSystem, opts: PlayOptions) {
        if opts.name.is_empty() {
            self.bgm_stop();
            return;
        }

        let volume = opts.volume_to_db();
        // no point playing a quiet sound
        if volume <= kira::Decibels::SILENCE {
            return;
        }

        let pitch = opts.pitch_to_playback_rate();
        let start_time = opts.position_to_start_time();
        let name = opts.name;

        if let Some(sound) = self.bgm.sound.as_mut().filter(|sound| sound.name == name) {
            sound.update(volume, pitch, opts.position);
            return;
        }

        let file = filesystem.open_file(&name).unwrap();
        let sound_data = SoundData::from_media_source(file)
            .unwrap()
            .loop_region(..)
            .volume(volume)
            .playback_rate(pitch)
            .start_time(start_time);

        let sound_handle = self.bgm.track_handle.play(sound_data).unwrap();
        self.bgm.sound = Some(Sound { sound_handle, name });
    }

    pub fn bgm_stop(&mut self) {
        if let Some(mut sound) = self.bgm.sound.take() {
            sound.sound_handle.stop(kira::Tween::default());
        }
    }

    pub fn bgs_play(&mut self, filesystem: &FileSystem, opts: PlayOptions) {
        if opts.name.is_empty() {
            self.bgs_stop();
            return;
        }

        let volume = opts.volume_to_db();
        // no point playing a quiet sound
        if volume <= kira::Decibels::SILENCE {
            return;
        }

        let pitch = opts.pitch_to_playback_rate();
        let start_time = opts.position_to_start_time();
        let name = opts.name;

        if let Some(sound) = self.bgs.sound.as_mut().filter(|sound| sound.name == name) {
            sound.update(volume, pitch, opts.position);
            return;
        }
        let file = filesystem.open_file(&name).unwrap();
        let sound_data = SoundData::from_media_source(file)
            .unwrap()
            .loop_region(..)
            .volume(volume)
            .playback_rate(pitch)
            .start_time(start_time);

        let sound_handle = self.bgs.track_handle.play(sound_data).unwrap();
        self.bgs.sound = Some(Sound { sound_handle, name });
    }

    pub fn bgs_stop(&mut self) {
        if let Some(mut sound) = self.bgs.sound.take() {
            sound.sound_handle.stop(kira::Tween::default());
        }
    }

    pub fn se_play(&mut self, filesystem: &FileSystem, opts: PlayOptions) {
        let free_index = self.playing_ses.iter().enumerate().find_map(|(i, s)| {
            let is_stopped = s.state() == kira::sound::PlaybackState::Stopped;
            is_stopped.then_some(i)
        });

        let is_full = self.playing_ses.capacity() == self.playing_ses.len();
        if free_index.is_none() && is_full {
            log::error!("SE queue is full!");
        }

        let volume = opts.volume_to_db();
        // no point playing a quiet sound
        if volume <= kira::Decibels::SILENCE {
            return;
        }

        let pitch = opts.pitch_to_playback_rate();
        let start_time = opts.position_to_start_time();
        let name = opts.name;

        let file = filesystem.open_file(&name).unwrap();
        let sound_data = SoundData::from_media_source(file)
            .unwrap()
            .volume(volume)
            .playback_rate(pitch)
            .start_time(start_time);
        let sound_handle = self.se_track.play(sound_data).unwrap();

        if let Some(free_index) = free_index {
            self.playing_ses[free_index] = sound_handle;
        } else {
            // push to the back of the vec because either
            // 1) its full
            // 2) it hasnt been filled yet
            self.playing_ses.push(sound_handle);
        }
    }

    pub fn se_stop(&mut self) {
        self.playing_ses.clear();
    }
}

impl symphonia::core::io::MediaSource for Box<dyn File> {
    fn is_seekable(&self) -> bool {
        true
    }

    fn byte_len(&self) -> Option<u64> {
        self.file_len()
    }
}
