use bevy::audio::AudioSource;

use crate::DspSource;

impl DspSource {
    /// Convert this to an audio source.
    #[must_use]
    pub fn into_audio_source(self, sample_rate: f64) -> AudioSource {
        let bytes: std::sync::Arc<[u8]> = self.generate_raw_bytes(sample_rate).into();

        AudioSource { bytes }
    }
}