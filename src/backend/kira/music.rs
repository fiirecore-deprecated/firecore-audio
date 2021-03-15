use kira::instance::InstanceSettings;
use kira::instance::StopInstanceSettings;
use kira::instance::handle::InstanceHandle;
use kira::sound::handle::SoundHandle;
use dashmap::DashMap;
use parking_lot::Mutex;
use firecore_audio_lib::music::{MusicId, MusicData};

use crate::error::PlayAudioError;

lazy_static::lazy_static! {
    pub static ref MUSIC_MAP: DashMap<MusicId, (MusicData, SoundHandle)> = DashMap::new();
    pub static ref CURRENT_MUSIC: Mutex<Option<(MusicId, InstanceHandle)>> = Mutex::new(None);
}

pub fn play_music(music: MusicId) -> Result<(), PlayAudioError> {
    match CURRENT_MUSIC.try_lock() {
        Some(mut current) => {
            if let Some((_, mut instance)) = current.take() {
                if let Err(err) = instance.stop(StopInstanceSettings::default()) {
                    return Err(PlayAudioError::CurrentError(err));
                }
            }
        }
        None => {
            return Err(PlayAudioError::CurrentLocked);
        }
    }
    match MUSIC_MAP.get_mut(&music) {
        Some(mut music) => {
            match CURRENT_MUSIC.try_lock() {
                Some(mut current) => {
                    let loop_start = music.0.loop_start.unwrap_or_default();
                    match music.1.play(InstanceSettings {
                        loop_start: kira::instance::InstanceLoopStart::Custom(loop_start),
                        ..Default::default()
                    }) {
                        Ok(instance) => {
                            *current = Some((*music.key(), instance));
                            Ok(())
                        }
                        Err(err) => {
                            Err(PlayAudioError::PlayError(err))
                        }
                    }
                }
                None => {
                    Err(PlayAudioError::CurrentLocked)
                }
            }
            
        }
        None => {
            Err(PlayAudioError::Missing)
        }
    }   
}

pub fn get_current_music() -> Option<MusicId> {
    CURRENT_MUSIC.lock().as_ref().map(|(id, _)| *id)
}