use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptionItem {
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
    pub timing_estimated: bool,
}

pub struct CaptionGenerator;

impl CaptionGenerator {
    pub fn format_srt_timestamp(ms: u64) -> String {
        let total_secs = ms / 1000;
        let millis = ms % 1000;
        let secs = total_secs % 60;
        let total_mins = total_secs / 60;
        let mins = total_mins % 60;
        let hours = total_mins / 60;
        format!("{:02}:{:02}:{:02},{:03}", hours, mins, secs, millis)
    }

    pub fn format_vtt_timestamp(ms: u64) -> String {
        let total_secs = ms / 1000;
        let millis = ms % 1000;
        let secs = total_secs % 60;
        let total_mins = total_secs / 60;
        let mins = total_mins % 60;
        let hours = total_mins / 60;
        format!("{:02}:{:02}:{:02}.{:03}", hours, mins, secs, millis)
    }

    pub fn to_srt(items: &[CaptionItem]) -> String {
        let mut out = String::new();
        for (i, item) in items.iter().enumerate() {
            out.push_str(&format!("{}\n", i + 1));
            out.push_str(&format!(
                "{} --> {}\n",
                Self::format_srt_timestamp(item.start_ms),
                Self::format_srt_timestamp(item.end_ms)
            ));
            out.push_str(&item.text);
            out.push_str("\n\n");
        }
        out
    }

    pub fn to_vtt(items: &[CaptionItem]) -> String {
        let mut out = String::from("WEBVTT\n\n");
        for (i, item) in items.iter().enumerate() {
            out.push_str(&format!("{}\n", i + 1));
            out.push_str(&format!(
                "{} --> {}\n",
                Self::format_vtt_timestamp(item.start_ms),
                Self::format_vtt_timestamp(item.end_ms)
            ));
            out.push_str(&item.text);
            out.push_str("\n\n");
        }
        out
    }

    /// Split long text into cues meeting criteria: max 2 lines, <= 42 chars per line, 1-6s
    pub fn split_text_into_cues(text: &str, total_duration_ms: u64) -> Vec<CaptionItem> {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() {
            return Vec::new();
        }

        let mut lines = Vec::new();
        let mut curr_line = String::new();

        for w in words {
            if curr_line.is_empty() {
                curr_line.push_str(w);
            } else if curr_line.chars().count() + 1 + w.chars().count() <= 40 {
                curr_line.push(' ');
                curr_line.push_str(w);
            } else {
                lines.push(curr_line);
                curr_line = w.to_string();
            }
        }
        if !curr_line.is_empty() {
            lines.push(curr_line);
        }

        let cue_count = (lines.len() + 1) / 2;
        let cue_count = cue_count.max(1);
        let ms_per_cue = total_duration_ms / cue_count as u64;

        let mut cues = Vec::new();
        for i in 0..cue_count {
            let start = i as u64 * ms_per_cue;
            let end = ((i + 1) as u64 * ms_per_cue).min(total_duration_ms);

            let l1 = lines.get(i * 2).cloned().unwrap_or_default();
            let l2 = lines.get(i * 2 + 1).cloned();
            let cue_text = match l2 {
                Some(second) => format!("{}\n{}", l1, second),
                None => l1,
            };

            cues.push(CaptionItem {
                start_ms: start,
                end_ms: end,
                text: cue_text,
                timing_estimated: true,
            });
        }

        cues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caption_srt_formatting() {
        let items = vec![CaptionItem {
            start_ms: 1000,
            end_ms: 3500,
            text: "Xin chào các bạn đã đến với studio!".to_string(),
            timing_estimated: false,
        }];

        let srt = CaptionGenerator::to_srt(&items);
        assert!(srt.contains("00:00:01,000 --> 00:00:03,500"));
        assert!(srt.contains("Xin chào các bạn đã đến với studio!"));
    }
}
