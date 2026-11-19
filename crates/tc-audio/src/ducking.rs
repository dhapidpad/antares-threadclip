pub struct DuckingFilterBuilder;

impl DuckingFilterBuilder {
    /// Builds an FFmpeg complex audio filter graph that ducks music when voice is present.
    /// Standard: 48kHz, -14 LUFS integrated, -1 dBTP true peak, 10-15dB reduction (ratio 8, attack 50ms, release 300ms).
    pub fn build_ducking_filter(music_input_tag: &str, voice_input_tag: &str, output_tag: &str) -> String {
        format!(
            "[{voice}]asplit=2[v_main][v_sc];\
             [{music}][v_sc]sidechaincompress=threshold=0.05:ratio=8:attack=50:release=300[music_duck];\
             [music_duck][v_main]amix=inputs=2:normalize=0,aresample=48000,loudnorm=I=-14:TP=-1:LRA=11[{out}]",
            voice = voice_input_tag,
            music = music_input_tag,
            out = output_tag
        )
    }

    pub fn build_voice_only_filter(voice_input_tag: &str, output_tag: &str) -> String {
        format!(
            "[{voice}]aresample=48000,loudnorm=I=-14:TP=-1:LRA=11[{out}]",
            voice = voice_input_tag,
            out = output_tag
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ducking_filter_syntax() {
        let filter = DuckingFilterBuilder::build_ducking_filter("0:a", "1:a", "a_out");
        assert!(filter.contains("sidechaincompress"));
        assert!(filter.contains("loudnorm=I=-14"));
        assert!(filter.contains("aresample=48000"));
    }
}
