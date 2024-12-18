use std::{collections::HashMap, sync::RwLock};

use arraydeque::{ArrayDeque, Wrapping};
use daichi::serenity::GuildId;
use once_cell::sync::OnceCell;

type VoiceData = ArrayDeque<Vec<i16>, 600, Wrapping>;

static VOICECACHE: OnceCell<VoiceCache> = OnceCell::new();

pub struct VoiceCache {
    data: RwLock<HashMap<GuildId, VoiceData>>,
}

impl VoiceCache {
    fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    fn global() -> &'static Self {
        VOICECACHE.get_or_init(Self::new)
    }

    pub fn add(guild_id: GuildId, voice_data: Vec<i16>) {
        let mut data = Self::global().data.write().unwrap();
        let cache = data.entry(guild_id).or_default();
        cache.push_back(voice_data);
    }

    pub fn get(guild_id: &GuildId) -> Option<Vec<i16>> {
        match Self::global().data.read().unwrap().get(guild_id) {
            Some(data) => {
                let voice_data: Vec<Vec<i16>> = data.iter().cloned().collect();
                Some(Self::exponential_moving_average(
                    &Self::gaussian_filter(&voice_data.concat(), 1.5),
                    0.07,
                ))
            }
            None => None,
        }
    }

    pub fn clear(guild_id: &GuildId) {
        Self::global()
            .data
            .write()
            .unwrap()
            .remove(guild_id)
            .unwrap();
    }

    fn exponential_moving_average(audio: &[i16], alpha: f32) -> Vec<i16> {
        let mut smoothed_audio = Vec::with_capacity(audio.len());
        let mut previous = audio[0] as f32;

        for &sample in audio.iter() {
            previous = alpha * sample as f32 + (1.0 - alpha) * previous;
            smoothed_audio.push(previous.round() as i16);
        }

        smoothed_audio
    }

    #[allow(clippy::needless_range_loop)]
    fn gaussian_filter(audio: &[i16], sigma: f32) -> Vec<i16> {
        let kernel_size = (6.0 * sigma).ceil() as usize | 1; // ensure odd size
        let mut kernel = vec![0.0; kernel_size];
        let mut smoothed_audio = Vec::with_capacity(audio.len());

        // Create the Gaussian kernel
        let mut sum = 0.0;
        for i in 0..kernel_size {
            let x = i as f32 - (kernel_size as f32 / 2.0);
            kernel[i] = (-x * x / (2.0 * sigma * sigma)).exp();
            sum += kernel[i];
        }
        for i in 0..kernel_size {
            kernel[i] /= sum; // normalize the kernel
        }

        // Apply the filter to the audio
        for i in 0..audio.len() {
            let mut smoothed_sample = 0.0;
            for j in 0..kernel_size {
                let index = if i + j < kernel_size / 2 {
                    0
                } else if i + j >= audio.len() {
                    audio.len() - 1
                } else {
                    i + j - kernel_size / 2
                };
                smoothed_sample += kernel[j] * audio[index] as f32;
            }
            smoothed_audio.push(smoothed_sample.round() as i16);
        }

        smoothed_audio
    }
}
