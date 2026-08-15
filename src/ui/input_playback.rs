use std::time::Instant;

use rkg_utils::ControllerInput;
use rkg_utils::input_data::DriftFlag;

/// Mario Kart Wii runs at 59.94 fps. Each input frame lasts `1.0 / FRAME_RATE` seconds.
pub const FRAME_RATE: f64 = 59.94;

/// Input frame at which the "GO!" countdown ends and the in-game
/// timer/lap counting actually starts, i.e. `0:00.000`.
pub const RACE_START_FRAME: u32 = 240;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackSpeed {
    Quarter,
    Half,
    Normal,
    Double,
    Quadruple,
}

pub const PLAYBACK_SPEEDS: [PlaybackSpeed; 5] = [
    PlaybackSpeed::Quarter,
    PlaybackSpeed::Half,
    PlaybackSpeed::Normal,
    PlaybackSpeed::Double,
    PlaybackSpeed::Quadruple,
];

impl PlaybackSpeed {
    pub fn multiplier(self) -> f64 {
        match self {
            PlaybackSpeed::Quarter => 0.25,
            PlaybackSpeed::Half => 0.5,
            PlaybackSpeed::Normal => 1.0,
            PlaybackSpeed::Double => 2.0,
            PlaybackSpeed::Quadruple => 4.0,
        }
    }
}

impl std::fmt::Display for PlaybackSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PlaybackSpeed::Quarter => "0.25x",
            PlaybackSpeed::Half => "0.5x",
            PlaybackSpeed::Normal => "1x",
            PlaybackSpeed::Double => "2x",
            PlaybackSpeed::Quadruple => "4x",
        };
        f.write_str(s)
    }
}

/// Playback position and transport state for a single ghost's input data.
///
/// Frame numbers are 1-based, matching [`rkg_utils::InputData`]'s frame
/// ranges. `current_frame` is always kept in `1..=total_frames`.
#[derive(Debug, Clone)]
pub struct InputPlayback {
    pub current_frame: u32,
    pub is_playing: bool,
    pub speed: PlaybackSpeed,
    play_started_at: Option<Instant>,
    frame_at_play_start: u32,
}

impl InputPlayback {
    pub fn new() -> Self {
        Self {
            current_frame: 1,
            is_playing: false,
            speed: PlaybackSpeed::Normal,
            play_started_at: None,
            frame_at_play_start: 1,
        }
    }

    pub fn toggle_play(&mut self, total_frames: u32) {
        if self.is_playing {
            self.pause();
            return;
        }

        if self.current_frame >= total_frames {
            self.current_frame = 1;
        }
        self.frame_at_play_start = self.current_frame;
        self.play_started_at = Some(Instant::now());
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
        self.play_started_at = None;
    }


    pub fn tick(&mut self, total_frames: u32) {
        let Some(started_at) = self.play_started_at else {
            return;
        };

        let elapsed_secs = started_at.elapsed().as_secs_f64();
        let frames_elapsed = (elapsed_secs * FRAME_RATE * self.speed.multiplier()).floor() as u32;
        let new_frame = self
            .frame_at_play_start
            .saturating_add(frames_elapsed)
            .min(total_frames);

        self.current_frame = new_frame.max(1);

        if new_frame >= total_frames {
            self.pause();
        }
    }

    pub fn seek(&mut self, frame: u32, total_frames: u32) {
        self.current_frame = frame.clamp(1, total_frames.max(1));
        if self.is_playing {
            self.frame_at_play_start = self.current_frame;
            self.play_started_at = Some(Instant::now());
        }
    }

    pub fn step(&mut self, delta: i32, total_frames: u32) {
        self.pause();
        let new_frame =
            (self.current_frame as i64 + delta as i64).clamp(1, total_frames.max(1) as i64);
        self.current_frame = new_frame as u32;
    }

    pub fn jump_to_start(&mut self) {
        self.pause();
        self.current_frame = 1;
    }

    pub fn jump_to_end(&mut self, total_frames: u32) {
        self.pause();
        self.current_frame = total_frames.max(1);
    }

    pub fn set_speed(&mut self, speed: PlaybackSpeed) {
        self.speed = speed;
        if self.is_playing {
            self.frame_at_play_start = self.current_frame;
            self.play_started_at = Some(Instant::now());
        }
    }
}

impl Default for InputPlayback {
    fn default() -> Self {
        Self::new()
    }
}

pub fn effective_drift_flags(inputs: &[ControllerInput]) -> Vec<bool> {
    let mut result = Vec::with_capacity(inputs.len());
    let mut simulated = false;
    for (idx, input) in inputs.iter().enumerate() {
        let accel = input.accelerator();
        let brake = input.brake();
        let (prev_accel, prev_brake) = if idx > 0 {
            (inputs[idx - 1].accelerator(), inputs[idx - 1].brake())
        } else {
            (false, false)
        };
        if !brake {
            simulated = false;
        } else if (prev_accel || accel) && !prev_brake && brake {
            simulated = true;
        } else if accel && !prev_accel && prev_brake {
            simulated = false;
        }
        result.push(match input.drift_flag() {
            DriftFlag::Enabled => true,
            DriftFlag::Disabled => false,
            DriftFlag::AutoDetect => simulated,
        });
    }
    result
}
