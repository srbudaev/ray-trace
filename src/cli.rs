use std::env;

pub struct Config {
    pub num_frames: usize,
    pub scene: usize,
}

impl Config {
    pub fn from_args() -> Self {
        let args: Vec<String> = env::args().collect();
        
        let mut num_frames = 1; // Default: 1 frame
        let mut scene = 0; // Default: scene 0 (random_scene)
        
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--scene" | "-s" => {
                    if i + 1 < args.len() {
                        scene = args[i + 1].parse().unwrap_or(0);
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "--frames" | "-f" => {
                    if i + 1 < args.len() {
                        num_frames = args[i + 1].parse().unwrap_or(1);
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                _ => {
                    // Try to parse as number (for backward compatibility)
                    if let Ok(n) = args[i].parse::<usize>() {
                        num_frames = n;
                    }
                    i += 1;
                }
            }
        }
        
        Self { num_frames, scene }
    }
    
    pub fn print_usage() {
        eprintln!("Usage: cargo run -- [OPTIONS]");
        eprintln!("   or: ./target/debug/ray-trace [OPTIONS]");
        eprintln!("");
        eprintln!("Options:");
        eprintln!("  --scene, -s <N>    Select scene (0=random, 1=sphere, 2=plane+cube, 3=all objects, 4=all objects alt camera)");
        eprintln!("  --frames, -f <N>   Number of frames to render (default: 1)");
        eprintln!("");
        eprintln!("Examples:");
        eprintln!("  cargo run -- --scene 1              # Render scene 1 (sphere)");
        eprintln!("  cargo run -- --scene 2 --frames 1   # Render scene 2, 1 frame");
        eprintln!("  cargo run -- 1                     # Render default scene, 1 frame (backward compatible)");
        eprintln!("");
        eprintln!("  ./target/debug/ray-trace --scene 1  # After building, run directly");
    }
}

