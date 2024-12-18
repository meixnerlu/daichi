use daichi::*;
use songbird::events::context_data::VoiceTick;

use super::VoiceHandler;

pub async fn handle_voice_tick(handler: &VoiceHandler, tick: &VoiceTick) -> Result<()> {
    if tick.speaking.is_empty() {
        handler
            .get_inner()
            .lock()
            .unwrap()
            .handle_new_tick(generate_silent_audio());

        return Ok(());
    }

    let mut tickdata: Vec<Vec<i16>> = tick
        .speaking
        .values()
        .map(|data| data.decoded_voice.clone().unwrap())
        .collect();
    tickdata.sort_by_key(|v| std::cmp::Reverse(v.len()));

    let cols = tickdata[0].len();
    let collapsed: Vec<i16> = (0..cols)
        .map(|i| tickdata.iter().map(|inner| inner[i]).sum())
        .collect();

    handler
        .get_inner()
        .lock()
        .unwrap()
        .handle_new_tick(collapsed);

    Ok(())
}

fn generate_silent_audio() -> Vec<i16> {
    let duration_ms = 20;
    let sample_rate = 48_000;

    let num_samples = (sample_rate as f32 * (duration_ms as f32 / 1000.0)) as usize;
    vec![0; num_samples] // Return a Vec filled with 0s, representing silence
}
