use firecore_audio_lib::serialized::SerializedMusicData;
use firecore_audio_lib::serialized::SerializedSoundData;
use kira::manager::AudioManager;
use kira::sound::SoundSettings;
use parking_lot::Mutex;

lazy_static::lazy_static! {
    pub static ref AUDIO_CONTEXT: Mutex<Option<AudioManager>> = Mutex::new(None);
}

pub fn create() -> Result<(), kira::manager::error::SetupError> {
    *AUDIO_CONTEXT.lock() = match AudioManager::new(kira::manager::AudioManagerSettings::default()) {
        Ok(am) => Some(am),
        Err(err) => return Err(err),
    };

    Ok(())
}

pub fn add_track(music_data: SerializedMusicData) {
    match super::from_ogg_bytes(&music_data.bytes, SoundSettings::default()) {
        Ok(sound) => match AUDIO_CONTEXT.lock().as_mut() {
            Some(manager) => {
                match manager.add_sound(sound) {
                    Ok(sound) => {
                        // println!("Added music");
                        super::music::MUSIC_MAP.insert(music_data.music.track, (music_data.music.data, sound));
                        // debug!("Loaded music \"{:?}\" successfully", music);
                    }
                    Err(err) => {
                        // eprintln!("{}", err);
                        // errors.push(AudioError::AddSoundError(err));
                        // warn!("Problem loading music \"{:?}\" with error {}", music, err);
                    }
                }
            }
            None => {
                // eprintln!("No audio manager");
            }
        }
        Err(err) => {
            // eprintln!("{}", err);
            // errors.push(AudioError::DecodeError(err));
            // warn!("Problem decoding bytes of \"{:?}\" in executable with error {}", music, err);
        }
    }
}

pub fn add_sound(sound_data: SerializedSoundData) {
    match super::from_ogg_bytes(&sound_data.bytes, SoundSettings::default()) {
        Ok(sound) => {
            match super::context::AUDIO_CONTEXT.lock().as_mut() {
                Some(context) => {
                    match context.add_sound(sound) {
                        Ok(sound) => {
                            super::sound::SOUND_MAP.insert(sound_data.sound, sound);
                            // return ok
                        }
                        Err(err) => {
                            // return err
                        }
                    }
                }
                None => {
                    // return err
                }
            }
        }
        Err(err) => {
            // return err
        }
    }
}