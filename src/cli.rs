use std::env;

pub struct Config {
    pub num_frames: usize,
}

impl Config {
    pub fn from_args() -> Self {
        let args: Vec<String> = env::args().collect();
        let num_frames = if args.len() > 1 {
            args[1].parse().unwrap_or(20)
        } else {
            60 // Default number of frames
        };
        Self { num_frames }
    }
}

