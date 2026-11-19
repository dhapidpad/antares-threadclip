use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RationalFps {
    pub num: u32,
    pub den: u32,
}

impl RationalFps {
    pub const FPS_30: Self = Self { num: 30, den: 1 };
    pub const FPS_60: Self = Self { num: 60, den: 1 };
    pub const FPS_24: Self = Self { num: 24, den: 1 };
    pub const FPS_25: Self = Self { num: 25, den: 1 };
    pub const FPS_29_97: Self = Self {
        num: 30000,
        den: 1001,
    };

    pub fn new(num: u32, den: u32) -> Result<Self, String> {
        if den == 0 {
            return Err("Denominator cannot be 0".to_string());
        }
        Ok(Self { num, den })
    }

    pub fn as_f64(&self) -> f64 {
        self.num as f64 / self.den as f64
    }

    pub fn to_ratio_string(&self) -> String {
        format!("{}/{}", self.num, self.den)
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        if let Some((num_str, den_str)) = s.split_once('/') {
            let num = num_str.trim().parse::<u32>().map_err(|e| e.to_string())?;
            let den = den_str.trim().parse::<u32>().map_err(|e| e.to_string())?;
            Self::new(num, den)
        } else {
            let float_val = s.trim().parse::<f64>().map_err(|e| e.to_string())?;
            if (float_val - 29.97).abs() < 0.01 {
                Ok(Self::FPS_29_97)
            } else {
                let rounded = float_val.round() as u32;
                Self::new(rounded, 1)
            }
        }
    }
}

impl fmt::Display for RationalFps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Frame(pub u64);

impl Frame {
    pub fn new(val: u64) -> Self {
        Self(val)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn to_ms(&self, fps: RationalFps) -> u64 {
        // ms = frame * 1000 * den / num
        (self.0 as u128 * 1000 * fps.den as u128 / fps.num as u128) as u64
    }

    pub fn from_ms(ms: u64, fps: RationalFps) -> Self {
        // frame = round(ms * num / (1000 * den))
        let num = ms as u128 * fps.num as u128;
        let den = 1000u128 * fps.den as u128;
        let frame = (num + (den / 2)) / den;
        Self(frame as u64)
    }

    pub fn to_seconds(&self, fps: RationalFps) -> f64 {
        self.0 as f64 * fps.den as f64 / fps.num as f64
    }

    pub fn from_seconds(secs: f64, fps: RationalFps) -> Self {
        let frames = (secs * fps.as_f64()).round();
        Self(frames.max(0.0) as u64)
    }

    pub fn to_timecode(&self, fps: RationalFps) -> Timecode {
        let fps_nominal = (fps.as_f64()).round() as u64;
        let fps_nominal = fps_nominal.max(1);

        let total_frames = self.0;
        let frame = total_frames % fps_nominal;
        let total_seconds = total_frames / fps_nominal;
        let seconds = total_seconds % 60;
        let total_minutes = total_seconds / 60;
        let minutes = total_minutes % 60;
        let hours = total_minutes / 60;

        Timecode {
            hours: hours as u32,
            minutes: minutes as u32,
            seconds: seconds as u32,
            frames: frame as u32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timecode {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub frames: u32,
}

impl Timecode {
    pub fn new(hours: u32, minutes: u32, seconds: u32, frames: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
            frames,
        }
    }

    pub fn to_frame(&self, fps: RationalFps) -> Frame {
        let fps_nominal = (fps.as_f64()).round() as u64;
        let total_seconds =
            self.hours as u64 * 3600 + self.minutes as u64 * 60 + self.seconds as u64;
        let total_frames = total_seconds * fps_nominal + self.frames as u64;
        Frame(total_frames)
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 4 {
            return Err("Timecode format must be HH:MM:SS:FF".to_string());
        }
        let hours = parts[0].parse::<u32>().map_err(|e| e.to_string())?;
        let minutes = parts[1].parse::<u32>().map_err(|e| e.to_string())?;
        let seconds = parts[2].parse::<u32>().map_err(|e| e.to_string())?;
        let frames = parts[3].parse::<u32>().map_err(|e| e.to_string())?;
        Ok(Self {
            hours,
            minutes,
            seconds,
            frames,
        })
    }
}

impl fmt::Display for Timecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}:{:02}",
            self.hours, self.minutes, self.seconds, self.frames
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fps_parse() {
        let fps30 = RationalFps::parse("30/1").unwrap();
        assert_eq!(fps30.num, 30);
        assert_eq!(fps30.den, 1);

        let fps2997 = RationalFps::parse("30000/1001").unwrap();
        assert_eq!(fps2997.num, 30000);
        assert_eq!(fps2997.den, 1001);

        let fps60 = RationalFps::parse("60").unwrap();
        assert_eq!(fps60.num, 60);
    }

    #[test]
    fn test_frame_ms_roundtrip() {
        let fps = RationalFps::FPS_30;
        let test_frames = [0, 1, 15, 30, 60, 150, 900, 3600];
        for f in test_frames {
            let frame = Frame(f);
            let ms = frame.to_ms(fps);
            let back = Frame::from_ms(ms, fps);
            assert_eq!(frame, back, "Mismatch at frame {f}");
        }
    }

    #[test]
    fn test_timecode_roundtrip() {
        let fps = RationalFps::FPS_30;
        let tc = Timecode::new(1, 23, 45, 12);
        let frame = tc.to_frame(fps);
        let back = frame.to_timecode(fps);
        assert_eq!(tc, back);
        assert_eq!(tc.to_string(), "01:23:45:12");
    }
}
