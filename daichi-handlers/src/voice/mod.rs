use std::fs::File;

use daichi::*;
use hound::{SampleFormat, WavSpec, WavWriter};

mod cache;
mod events;
mod join;
mod leave;
mod play;
pub use cache::VoiceCache;
pub use events::*;
pub use join::*;
pub use leave::*;
pub use play::*;

pub fn convert_to_wav(data: Vec<i16>, guild_id: serenity::GuildId) -> Result<(File, String)> {
    let spec = WavSpec {
        channels: 2,
        sample_rate: 48_000,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let file_path = format!("/tmp/daichi/{}.wav", guild_id);

    let mut writer = WavWriter::create(&file_path, spec).map_err(Error::from_any)?;

    for sample in data {
        writer.write_sample(sample).map_err(Error::from_any)?;
    }

    writer.flush().map_err(Error::from_any)?;

    let tmp_file = File::open(&file_path).map_err(Error::from_any)?;
    Ok((tmp_file, file_path))
}
